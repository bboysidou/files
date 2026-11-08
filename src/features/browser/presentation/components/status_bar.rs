use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::helpers::format::item_count::item_count;
use crate::core::theme::colors::{DIVIDER, FG_BRIGHT, FG_MUTED};
use crate::core::theme::metrics::{STATUS_GAP, STATUS_PAD_X, STATUS_PAD_Y, TEXT_STATUS};

#[derive(IntoElement)]
pub struct StatusBar {
    pub location: SharedString,
    pub item_count: usize,
    pub selected_count: usize,
}

impl RenderOnce for StatusBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bar = div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(STATUS_GAP))
            .px(px(STATUS_PAD_X))
            .py(px(STATUS_PAD_Y))
            .border_t_1()
            .border_color(rgb(DIVIDER))
            .text_size(px(TEXT_STATUS))
            .text_color(rgb(FG_MUTED))
            .child(div().text_color(rgb(FG_BRIGHT)).child(self.location))
            .child(item_count(self.item_count));
        if self.selected_count == 0 {
            return bar;
        }
        bar.child(t!("status.selected", count = self.selected_count).to_string())
    }
}
