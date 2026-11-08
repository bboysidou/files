use std::rc::Rc;

use gpui::{Context, Window};

use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::browser_event::BrowserEvent;
use crate::global_components::handlers::IndexHandler;

const TEAR_MIN_TABS: usize = 2;

impl BrowserView {
    pub fn tear_active(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.tear_tab(self.active, window, cx);
    }

    fn tear_tab(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
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

    pub(super) fn tear_handler(cx: &mut Context<Self>) -> IndexHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |index, window, cx| {
            let _ = view.update(cx, |this, cx| this.tear_tab(index, window, cx));
        })
    }
}
