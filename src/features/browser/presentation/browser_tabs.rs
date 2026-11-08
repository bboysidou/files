use std::path::PathBuf;

use gpui::{App, Context, Focusable, Window};

use crate::core::constants::settings::PERMILLE;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;

use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::places::domain::entities::place::PlaceTarget;

impl BrowserView {
    pub fn notify_message(&mut self, message: String, cx: &mut Context<Self>) {
        self.show_toast(message, false, cx);
    }

    pub fn location(&self, cx: &App) -> PlaceTarget {
        self.active_pane().read(cx).location().clone()
    }

    pub fn navigate(&mut self, target: PlaceTarget, cx: &mut Context<Self>) {
        self.active_pane()
            .clone()
            .update(cx, |pane, cx| pane.navigate(target, cx));
    }

    pub(super) fn restore_tabs(
        &mut self,
        (tabs, active): (Vec<TabSnapshot>, usize),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let targets = if tabs.is_empty() {
            vec![TabSnapshot::fresh(self.setup.home_target.clone())]
        } else {
            tabs
        };
        for snapshot in targets {
            let tab = self.open_pane(snapshot, window, cx);
            self.tabs.push(tab);
        }
        self.active = active.min(self.tabs.len() - 1);
    }

    pub fn snapshot(&self, cx: &App) -> BrowserSnapshot {
        BrowserSnapshot {
            tabs: self
                .tabs
                .iter()
                .map(|tab| tab.pane.read(cx).tab_snapshot())
                .collect(),
            active: self.active,
            options: self.options,
            preview_open: self.column.open,
            dual_split: (self.dual.split * PERMILLE).round() as u16,
            restore_tabs: self.settings.restore_tabs,
            key_preset: self.settings.key_preset,
            folder_views: self.settings.folder_views.clone(),
        }
    }

    pub(super) fn new_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let target = match self.location(cx) {
            PlaceTarget::Overview => self.setup.home_target.clone(),
            target => target,
        };
        let tab = self.open_pane(TabSnapshot::fresh(target), window, cx);
        self.tabs.push(tab);
        self.sync_cut(cx);
        self.sync_compact(cx);
        self.activate(self.tabs.len() - 1, window, cx);
    }

    pub fn drop_start_placeholder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.tabs.len() != 2 {
            return;
        }
        let first = self.tabs[0].pane.read(cx);
        if *first.location() == self.setup.home_target && !first.has_history() {
            self.close_tab(0, window, cx);
        }
    }

    pub fn close_active(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.close_tab(self.active, window, cx);
    }

    pub(super) fn close_tab(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.tabs.len() {
            return;
        }
        if self.tabs.len() == 1 {
            let fresh = self.open_pane(
                TabSnapshot::fresh(self.setup.home_target.clone()),
                window,
                cx,
            );
            self.tabs = vec![fresh];
            self.activate(0, window, cx);
            return;
        }
        self.tabs.remove(index);
        let next = if self.active >= index && self.active > 0 {
            self.active - 1
        } else {
            self.active
        };
        self.activate(next.min(self.tabs.len() - 1), window, cx);
    }

    pub(super) fn activate(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(tab) = self.tabs.get(index) else {
            return;
        };
        self.active = index;
        window.focus(&tab.pane.focus_handle(cx), cx);
        self.sync_preview_column(cx);
        cx.notify();
    }

    pub(super) fn cycle(&mut self, delta: isize, window: &mut Window, cx: &mut Context<Self>) {
        let count = self.tabs.len() as isize;
        let next = (self.active as isize + delta).rem_euclid(count) as usize;
        self.activate(next, window, cx);
    }

    pub fn open_tab(
        &mut self,
        (target, focus): (PlaceTarget, Option<PathBuf>),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let tab = self.open_pane(TabSnapshot::fresh(target), window, cx);
        if let Some(path) = focus {
            tab.pane.update(cx, |pane, _| pane.focus_when_loaded(path));
        }
        self.tabs.push(tab);
        self.sync_cut(cx);
        self.sync_compact(cx);
        self.activate(self.tabs.len() - 1, window, cx);
    }
}
