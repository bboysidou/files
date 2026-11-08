use gpui::{
    AnyElement, App, FontWeight, IntoElement, RenderOnce, SharedString, Window, div, prelude::*,
    px, rgb,
};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{FG_BRIGHT, LINE};
use crate::core::theme::metrics::{
    QUICK_LOOK_HEADER_GAP, QUICK_LOOK_HEADER_PAD_X, QUICK_LOOK_HEADER_PAD_Y,
};
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct PreviewColumnHeader {
    pub icon: AnyElement,
    pub title: SharedString,
    pub on_quick_look: ClickHandler,
}

impl RenderOnce for PreviewColumnHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
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
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .text_ellipsis()
                    .whitespace_nowrap()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(FG_BRIGHT))
                    .child(self.title),
            )
            .child(IconButton::new("column-quick-look", Icon::Eye).on_click(self.on_quick_look))
    }
}
