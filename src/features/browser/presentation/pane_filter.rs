use std::rc::Rc;

use gpui::{Context, Focusable, Window};

use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::filter_model::FilterModel;
use crate::features::browser::presentation::state::load_state::LoadState;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::text_field::text_field_view::TextFieldEvent;

impl PaneView {
    pub(super) fn open_filter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.filter_open = true;
        window.focus(&self.filter_field.focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn on_filter_event(
        &mut self,
        event: TextFieldEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            TextFieldEvent::Changed => {
                self.filter = self.filter_field.read(cx).text().to_string();
                self.rearrange(cx);
            }
            TextFieldEvent::Submitted => {
                self.filter_open = !self.filter.is_empty();
                window.focus(&self.focus_handle, cx);
                cx.notify();
            }
            TextFieldEvent::Cancelled => self.clear_filter(window, cx),
        }
    }

    pub(super) fn clear_filter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.reset_filter(cx);
        window.focus(&self.focus_handle, cx);
        self.rearrange(cx);
    }

    pub(super) fn reset_filter(&mut self, cx: &mut Context<Self>) {
        self.filter.clear();
        self.filter_open = false;
        self.filter_field.update(cx, |field, cx| field.clear(cx));
    }

    pub(super) fn filter_model(&self) -> Option<FilterModel> {
        self.filter_open.then(|| FilterModel {
            field: self.filter_field.clone(),
            shown: self.entries.len(),
            total: self.unfiltered_count(),
        })
    }

    pub(super) fn open_filter_handler(cx: &mut Context<Self>) -> ClickHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.open_filter(window, cx));
        })
    }

    fn unfiltered_count(&self) -> usize {
        match &self.load {
            LoadState::Ready(raw) => raw
                .iter()
                .filter(|entry| self.show_hidden || !entry.hidden)
                .count(),
            _ => 0,
        }
    }
}
