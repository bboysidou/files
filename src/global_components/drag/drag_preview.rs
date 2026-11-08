use gpui::{Context, IntoElement, Render, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT};
use crate::core::theme::metrics::{CONTROL_RADIUS, NAV_PAD_X, TEXT_SMALL};

pub struct DragPreview {
    pub label: SharedString,
}

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px(px(NAV_PAD_X))
            .py(px(NAV_PAD_X / 2.0))
            .rounded(px(CONTROL_RADIUS))
            .border_1()
            .border_color(rgb(ACCENT))
            .bg(rgb(CARD))
            .text_size(px(TEXT_SMALL))
            .text_color(rgb(FG_BRIGHT))
            .child(self.label.clone())
    }
}
