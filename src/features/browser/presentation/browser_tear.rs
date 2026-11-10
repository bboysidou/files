use gpui::{Context, Window};

use crate::core::constants::files::TEAR_MIN_TABS;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::browser_event::BrowserEvent;

impl BrowserView {
    pub fn tear_active(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.tear_tab(self.active, window, cx);
    }

    pub(super) fn tear_tab(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.tabs.len() < TEAR_MIN_TABS {
            return;
        }
        let Some(snapshot) = self
            .tabs
            .get(index)
            .map(|tab| tab.pane.read(cx).tab_snapshot())
        else {
            return;
        };
        self.close_tab(index, window, cx);
        cx.emit(BrowserEvent::TabDetached(snapshot));
    }
}
