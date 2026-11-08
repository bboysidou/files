use std::time::Duration;

use gpui::{App, Context};

use crate::core::constants::settings::SETTINGS_SAVE_DEBOUNCE_MS;
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;
use crate::features::settings::domain::entities::settings::Settings;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::saved_tab_mapping::{restored_tab, saved_tab};

impl ShellView {
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
            restore_tabs: settings.restore_tabs,
            folder_views: settings.folder_views.clone(),
            options: ViewOptions {
                show_hidden: settings.show_hidden,
                sort: SortOrder {
                    key: settings.sort_key,
                    ascending: settings.sort_ascending,
                    hidden_last: settings.hidden_last,
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
            folder_views: snapshot.folder_views.clone(),
            view_mode: snapshot.options.view,
            columns: snapshot.options.columns,
            sidebar_open: self.rail.open,
            preview_open: snapshot.preview_open,
            active_tab: snapshot.active,
            tabs: snapshot.tabs.iter().map(saved_tab).collect(),
        }
    }

    pub(super) fn persist(&mut self, cx: &mut Context<Self>) {
        let settings = self.current_settings(cx);
        if settings == self.saved {
            return;
        }
        self.saved = settings.clone();
        let repository = self.settings_repository.clone();
        self.save_task = Some(cx.spawn(async move |_, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(SETTINGS_SAVE_DEBOUNCE_MS))
                .await;
            cx.background_executor()
                .spawn(async move { repository.save(&settings) })
                .await;
        }));
    }

    pub(super) fn flush(&mut self, cx: &mut Context<Self>) {
        self.save_task = None;
        let settings = self.current_settings(cx);
        self.settings_repository.save(&settings);
        self.saved = settings;
    }
}
