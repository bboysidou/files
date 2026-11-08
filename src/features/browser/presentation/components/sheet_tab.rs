use gpui::{
    App, IntoElement, MouseButton, RenderOnce, SharedString, Window, div, prelude::*, px, rgb,
};

use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT, FG_MUTED, FG_SOFT};
use crate::core::theme::metrics::{
    TAB_GAP, TAB_HEIGHT, TAB_INDICATOR, TAB_PAD_LEFT, TAB_PAD_RIGHT, TAB_RADIUS, TEXT_TINY,
};
use crate::features::browser::presentation::components::tab_close_button::TabCloseButton;
use crate::global_components::handlers::IndexHandler;

#[derive(IntoElement)]
pub struct SheetTab {
    pub index: usize,
    pub title: SharedString,
    pub active: bool,
    pub on_select: IndexHandler,
    pub on_close: IndexHandler,
}

impl RenderOnce for SheetTab {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let index = self.index;
        let (close_click, close_middle, select) =
            (self.on_close.clone(), self.on_close, self.on_select);
        let close = TabCloseButton {
            index,
            on_close: close_click,
        };
        let number = div()
            .text_size(px(TEXT_TINY))
            .text_color(rgb(FG_MUTED))
            .child((self.index + 1).to_string());
        let tab = div()
            .id(("tab", self.index))
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(TAB_GAP))
            .h(px(TAB_HEIGHT))
            .pl(px(TAB_PAD_LEFT))
            .pr(px(TAB_PAD_RIGHT))
            .rounded_tl(px(TAB_RADIUS))
            .rounded_tr(px(TAB_RADIUS))
            .whitespace_nowrap()
            .cursor_pointer()
            .on_click(move |_, window, cx| select(index, window, cx))
            .on_mouse_down(MouseButton::Middle, move |_, window, cx| {
                close_middle(index, window, cx)
            });
        let tab = if self.active {
            tab.bg(rgb(CARD))
                .text_color(rgb(FG_BRIGHT))
                .border_b(px(TAB_INDICATOR))
                .border_color(rgb(ACCENT))
        } else {
            tab.text_color(rgb(FG_SOFT))
        };
        tab.child(self.title).child(number).child(close)
    }
}
