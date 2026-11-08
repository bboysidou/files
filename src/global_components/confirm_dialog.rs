use gpui::{
    App, FontWeight, IntoElement, MouseButton, RenderOnce, SharedString, Window, div, prelude::*,
    px, rgb,
};

use crate::core::theme::colors::{CARD, FG_BRIGHT, FG_SOFT, LINE_STRONG};
use crate::core::theme::metrics::{
    CONFIRM_GAP, CONFIRM_PAD, CONFIRM_WIDTH, QUICK_LOOK_RADIUS, TEXT_EMPTY_TITLE, TEXT_SMALL,
};
use crate::global_components::dialog_button::DialogButton;
use crate::global_components::handlers::ClickHandler;

#[derive(IntoElement)]
pub struct ConfirmDialog {
    pub title: SharedString,
    pub body: SharedString,
    pub cancel_label: SharedString,
    pub confirm_label: SharedString,
    pub on_cancel: ClickHandler,
    pub on_confirm: ClickHandler,
}

impl RenderOnce for ConfirmDialog {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let buttons = div()
            .flex()
            .justify_end()
            .gap(px(CONFIRM_GAP))
            .child(DialogButton {
                id: "confirm-cancel",
                label: self.cancel_label,
                danger: false,
                on_click: self.on_cancel,
            })
            .child(DialogButton {
                id: "confirm-accept",
                label: self.confirm_label,
                danger: true,
                on_click: self.on_confirm,
            });
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
            .child(buttons)
    }
}
