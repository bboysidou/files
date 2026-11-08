use gpui::{
    AnyElement, App, FontWeight, IntoElement, RenderOnce, SharedString, Window, div, prelude::*,
    px, rgb,
};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{FG_BRIGHT, FG_MUTED, LINE};
use crate::core::theme::metrics::{
    QUICK_LOOK_HEADER_GAP, QUICK_LOOK_HEADER_PAD_X, QUICK_LOOK_HEADER_PAD_Y, TEXT_STATUS,
};
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct QuickLookHeader {
    pub icon: AnyElement,
    pub title: SharedString,
    pub on_close: ClickHandler,
}

impl RenderOnce for QuickLookHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let title = div()
            .flex_1()
            .min_w_0()
            .overflow_hidden()
            .text_ellipsis()
            .whitespace_nowrap()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(FG_BRIGHT))
            .child(self.title);
        let hint = div()
            .text_size(px(TEXT_STATUS))
            .text_color(rgb(FG_MUTED))
            .child(t!("ql.hint").to_string());
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(QUICK_LOOK_HEADER_GAP))
            .px(px(QUICK_LOOK_HEADER_PAD_X))
            .py(px(QUICK_LOOK_HEADER_PAD_Y))
            .border_b_1()
            .border_color(rgb(LINE))
            .child(self.icon)
            .child(title)
            .child(hint)
            .child(IconButton::new("quick-look-close", Icon::Close).on_click(self.on_close))
    }
}
