use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{FG, FG_BRIGHT, FRAME, LINE, LINE_STRONG};
use crate::core::theme::metrics::{
    TEXT_SMALL, TEXT_STATUS, TOAST_BUTTON_HEIGHT, TOAST_BUTTON_PAD_X, TOAST_GAP, TOAST_PAD_LEFT,
    TOAST_PAD_RIGHT, TOAST_PAD_Y,
};
use crate::global_components::handlers::ClickHandler;

#[derive(IntoElement)]
pub struct Toast {
    pub message: SharedString,
    pub action: Option<(SharedString, ClickHandler)>,
}

impl RenderOnce for Toast {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let pill = div()
            .flex()
            .items_center()
            .gap(px(TOAST_GAP))
            .pl(px(TOAST_PAD_LEFT))
            .pr(px(TOAST_PAD_RIGHT))
            .py(px(TOAST_PAD_Y))
            .rounded_full()
            .bg(rgb(FRAME))
            .border_1()
            .border_color(rgb(LINE_STRONG))
            .text_size(px(TEXT_SMALL))
            .text_color(rgb(FG_BRIGHT))
            .child(self.message);
        let Some((label, on_action)) = self.action else {
            return pill;
        };
        pill.child(
            div()
                .id("toast-action")
                .flex()
                .items_center()
                .h(px(TOAST_BUTTON_HEIGHT))
                .px(px(TOAST_BUTTON_PAD_X))
                .rounded_full()
                .border_1()
                .border_color(rgb(LINE_STRONG))
                .text_size(px(TEXT_STATUS))
                .text_color(rgb(FG))
                .cursor_pointer()
                .hover(|style| style.bg(rgb(LINE)))
                .on_click(move |_, window, cx| on_action(window, cx))
                .child(label),
        )
    }
}
