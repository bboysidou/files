use gpui::{Context, Window};

use crate::features::browser::presentation::browser_view::BrowserView;

impl BrowserView {
    pub(super) fn move_tab(
        &mut self,
        (from, to): (usize, usize),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if from == to || from >= self.tabs.len() || to >= self.tabs.len() {
            return;
        }
        let active = self.tabs[self.active].pane.clone();
        let tab = self.tabs.remove(from);
        self.tabs.insert(to, tab);
        let index = self
            .tabs
            .iter()
            .position(|tab| tab.pane == active)
            .unwrap_or_default();
        self.activate(index, window, cx);
    }

    pub(super) fn shift_active_tab(
        &mut self,
        delta: isize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let from = self.focused_tab();
        let target = from as isize + delta;
        if target < 0 {
            return;
        }
        self.move_tab((from, target as usize), window, cx);
    }
}
