use std::time::Duration;

use gpui::{App, Context, EntityId};

use crate::core::constants::settings::SETTINGS_SAVE_DEBOUNCE_MS;
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::state::active_key_preset::ActiveKeyPreset;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;
use crate::features::browser::presentation::state::pins::Pins;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::settings::domain::entities::settings::{SavedWindow, Settings};
use crate::features::shell::presentation::app_key_bindings::bind_app_keys;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::open_shells::OpenShells;
use crate::features::shell::presentation::state::saved_tab_mapping::{restored_tab, saved_tab};
use crate::features::shell::presentation::state::shell_start::WindowRole;

impl ShellView {
    pub(super) fn starting_snapshot(
        saved: &Settings,
        role: WindowRole,
        tabs: Option<(Vec<TabSnapshot>, usize)>,
    ) -> BrowserSnapshot {
        let mut restore = Self::restored_snapshot(saved);
        match tabs {
            Some((tabs, active)) => {
                restore.active = active.min(tabs.len().saturating_sub(1));
                restore.tabs = tabs;
            }
            None if role == WindowRole::Extra => restore.tabs.clear(),
            None => {}
        }
        restore
    }

    pub(super) fn sync_key_preset(&mut self, cx: &mut Context<Self>) {
        let preset = self.browser.read(cx).key_preset();
        if cx.global::<ActiveKeyPreset>().0 != preset {
            bind_app_keys(preset, cx);
        }
    }

    pub(super) fn restored_snapshot(settings: &Settings) -> BrowserSnapshot {
        let saved = if settings.restore_tabs {
            settings.tabs.as_slice()
        } else {
            &[]
        };
        let tabs: Vec<_> = saved.iter().filter_map(restored_tab).collect();
        BrowserSnapshot {
            active: settings.active_tab.min(tabs.len().saturating_sub(1)),
            tabs,
            preview_open: settings.preview_open,
            dual_split: settings.dual_split,
            restore_tabs: settings.restore_tabs,
            key_preset: settings.key_preset,
            folder_views: settings.folder_views.clone(),
            options: ViewOptions {
                show_hidden: settings.show_hidden,
                sort: SortOrder {
                    key: settings.sort_key,
                    ascending: settings.sort_ascending,
                    hidden_last: settings.hidden_last,
                    seed: 0,
                },
                view: settings.view_mode,
                columns: settings.columns,
            },
        }
    }

    pub(super) fn current_settings(&self, cx: &App) -> Settings {
        let snapshot = self.browser.read(cx).snapshot(cx);
        Settings {
            show_hidden: snapshot.options.show_hidden,
            sort_key: snapshot.options.sort.key,
            sort_ascending: snapshot.options.sort.ascending,
            hidden_last: snapshot.options.sort.hidden_last,
            restore_tabs: snapshot.restore_tabs,
            key_preset: snapshot.key_preset,
            folder_views: snapshot.folder_views.clone(),
            servers: self.network.saved.clone(),
            pins: cx
                .try_global::<Pins>()
                .map(|pins| pins.0.clone())
                .unwrap_or_default(),
            view_mode: snapshot.options.view,
            columns: snapshot.options.columns,
            sidebar_open: self.rail.open,
            preview_open: snapshot.preview_open,
            dual_split: snapshot.dual_split,
            active_tab: snapshot.active,
            tabs: snapshot.tabs.iter().map(saved_tab).collect(),
            windows: Vec::new(),
        }
    }

    fn saved_window(&self, cx: &App) -> SavedWindow {
        let snapshot = self.browser.read(cx).snapshot(cx);
        SavedWindow {
            active_tab: snapshot.active,
            tabs: snapshot.tabs.iter().map(saved_tab).collect(),
        }
    }

    fn session_settings(&self, me: EntityId, keep_self: bool, cx: &App) -> Settings {
        let mut settings = self.current_settings(cx);
        let mut windows: Vec<SavedWindow> = cx
            .try_global::<OpenShells>()
            .map(|shells| {
                shells
                    .0
                    .iter()
                    .filter_map(|shell| match shell.view.upgrade() {
                        Some(view) if view.entity_id() == me => {
                            keep_self.then(|| self.saved_window(cx))
                        }
                        Some(view) => Some(view.read(cx).saved_window(cx)),
                        None => None,
                    })
                    .collect()
            })
            .unwrap_or_default();
        if windows.is_empty() {
            windows.push(self.saved_window(cx));
        }
        let first = windows.remove(0);
        settings.tabs = first.tabs;
        settings.active_tab = first.active_tab;
        settings.windows = windows;
        settings
    }

    fn share_prefs(&self, cx: &mut Context<Self>) {
        let prefs = self.browser.read(cx).snapshot(cx);
        let me = cx.entity_id();
        let others: Vec<_> = cx
            .try_global::<OpenShells>()
            .map(|shells| {
                shells
                    .0
                    .iter()
                    .filter_map(|shell| shell.view.upgrade())
                    .collect()
            })
            .unwrap_or_default();
        for other in others.into_iter().filter(|view| view.entity_id() != me) {
            other.update(cx, |other, cx| {
                other
                    .browser
                    .update(cx, |browser, cx| browser.apply_prefs(&prefs, cx))
            });
        }
    }

    pub(super) fn persist(&mut self, cx: &mut Context<Self>) {
        let settings = self.session_settings(cx.entity_id(), true, cx);
        if settings == self.saved {
            return;
        }
        if Self::prefs_of(&settings) != Self::prefs_of(&self.saved) {
            self.share_prefs(cx);
        }
        self.saved = settings;
        let repository = self.settings_repository.clone();
        self.save_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(SETTINGS_SAVE_DEBOUNCE_MS))
                .await;
            let Ok(settings) = this.update(cx, |this, cx| {
                this.session_settings(cx.entity_id(), true, cx)
            }) else {
                return;
            };
            cx.background_executor()
                .spawn(async move { repository.save(&settings) })
                .await;
        }));
    }

    fn prefs_of(settings: &Settings) -> Settings {
        Settings {
            tabs: Vec::new(),
            active_tab: 0,
            windows: Vec::new(),
            sidebar_open: true,
            ..settings.clone()
        }
    }

    pub(super) fn flush(&mut self, cx: &mut Context<Self>) {
        self.save_task = None;
        let settings = self.session_settings(cx.entity_id(), true, cx);
        self.settings_repository.save(&settings);
        self.saved = settings;
    }

    pub(super) fn flush_closing(&mut self, cx: &mut Context<Self>) {
        self.save_task = None;
        let settings = self.session_settings(cx.entity_id(), false, cx);
        if settings != self.saved {
            self.settings_repository.save(&settings);
        }
    }
}
