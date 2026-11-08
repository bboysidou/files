use gpui::{
    App, FontWeight, IntoElement, MouseButton, RenderOnce, SharedString, Window, div, prelude::*,
    px, rgb,
};

use crate::core::theme::colors::{CARD, FG_BRIGHT, FG_SOFT, LINE_STRONG};
use crate::core::theme::metrics::{
    CONFIRM_GAP, CONFIRM_PAD, CONFIRM_WIDTH, QUICK_LOOK_RADIUS, TEXT_EMPTY_TITLE, TEXT_SMALL,
};
use crate::global_components::dialog_button::DialogButton;

#[derive(IntoElement)]
pub struct ConfirmDialog {
    pub title: SharedString,
    pub body: SharedString,
    pub buttons: Vec<DialogButton>,
}

impl RenderOnce for ConfirmDialog {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(CONFIRM_GAP))
            .w(px(CONFIRM_WIDTH))
            .p(px(CONFIRM_PAD))
            .rounded(px(QUICK_LOOK_RADIUS))
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(LINE_STRONG))
            .shadow_lg()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .text_size(px(TEXT_EMPTY_TITLE))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(FG_BRIGHT))
                    .child(self.title),
            )
            .child(
                div()
                    .text_size(px(TEXT_SMALL))
                    .text_color(rgb(FG_SOFT))
                    .child(self.body),
            )
            .child(
                div()
                    .flex()
                    .justify_end()
                    .gap(px(CONFIRM_GAP))
                    .children(self.buttons),
            )
    }
}
