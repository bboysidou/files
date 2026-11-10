use gpui::{AnyElement, Context, EntityId, Focusable, IntoElement, Window, div, prelude::*, rgba};

use crate::core::constants::files::SPLIT_MIN_TABS;
use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::colors::{ACCENT, DROP_ALPHA};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::browser::presentation::state::tab::Tab;
use crate::global_components::drag::dragged_tab::DraggedTab;

impl BrowserView {
    pub(super) fn pane_body(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let body = self.dual_body(window, cx);
        let tint = rgba((ACCENT << 8) | DROP_ALPHA);
        div()
            .relative()
            .flex()
            .flex_1()
            .size_full()
            .min_h_0()
            .child(body)
            .drag_over::<DraggedTab>(move |style, _, _, _| style.bg(tint))
            .on_drop(cx.listener(|this, dragged: &DraggedTab, window, cx| {
                this.split_with_tab(dragged.index, window, cx);
            }))
            .into_any_element()
    }

    fn split_with_tab(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pane) = self.tabs.get(index).map(|tab| tab.pane.clone()) else {
            return;
        };
        if self.tabs.len() < SPLIT_MIN_TABS {
            return;
        }
        let before = match self.dual.linked {
            Some((_, before)) => before,
            None => self.options.view,
        };
        if index == self.active {
            let neighbour = if index > 0 { index - 1 } else { index + 1 };
            self.activate(neighbour, window, cx);
        }
        self.dual.linked = Some((pane.entity_id(), before));
        window.focus(&pane.focus_handle(cx), cx);
        self.dual.partner = Some(Tab {
            _changes: cx.observe(&pane, |_, _, _| {}),
            _events: cx.subscribe(&pane, |_, _, _: &PaneEvent, _| {}),
            pane,
        });
        self.dual.focused = true;
        self.set_view(ViewMode::Dual, cx);
        cx.notify();
    }

    pub(super) fn is_split_tab(&self, index: usize) -> bool {
        self.options.view == ViewMode::Dual
            && self.tabs.get(index).is_some_and(|tab| {
                self.dual
                    .linked
                    .is_some_and(|(linked, _)| linked == tab.pane.entity_id())
            })
    }

    pub(super) fn focus_split(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(partner) = &self.dual.partner {
            window.focus(&partner.pane.focus_handle(cx), cx);
        }
        self.dual.focused = true;
        self.sync_preview_column(cx);
        cx.notify();
    }

    pub(super) fn unsplit_if_linked(&mut self, closed: EntityId, cx: &mut Context<Self>) {
        let Some((source, before)) = self.dual.linked else {
            return;
        };
        if source != closed {
            return;
        }
        self.dual.linked = None;
        self.dual.partner = None;
        self.dual.focused = false;
        self.set_view(before, cx);
        cx.notify();
    }

    pub(super) fn focused_tab(&self) -> usize {
        let partner = self
            .dual
            .partner
            .as_ref()
            .filter(|_| self.dual.focused && self.options.view == ViewMode::Dual);
        partner
            .and_then(|partner| {
                let id = partner.pane.entity_id();
                self.tabs.iter().position(|tab| tab.pane.entity_id() == id)
            })
            .unwrap_or(self.active)
    }
}
