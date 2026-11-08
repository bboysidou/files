use std::path::PathBuf;

use gpui::{Context, Focusable, Window};

use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::places::domain::entities::place::PlaceTarget;

impl BrowserView {
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

    pub(super) fn close_active(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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
