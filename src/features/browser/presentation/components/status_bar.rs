use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::helpers::format::item_count::item_count;
use crate::core::theme::colors::{DANGER, DIVIDER, FG_BRIGHT, FG_MUTED};
use crate::core::theme::metrics::{
    ADMIN_CHIP_PAD_X, ADMIN_CHIP_RADIUS, STATUS_GAP, STATUS_PAD_X, STATUS_PAD_Y, TEXT_STATUS,
};
use crate::global_components::handlers::ClickHandler;

#[derive(IntoElement)]
pub struct StatusBar {
    pub location: SharedString,
    pub item_count: usize,
    pub selected_count: usize,
    pub on_end_admin: Option<ClickHandler>,
}

impl RenderOnce for StatusBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let admin = self.on_end_admin.map(|on_end| {
            div()
                .id("admin-session")
                .ml_auto()
                .px(px(ADMIN_CHIP_PAD_X))
                .rounded(px(ADMIN_CHIP_RADIUS))
                .border_1()
                .border_color(rgb(DANGER))
                .text_color(rgb(DANGER))
                .cursor_pointer()
                .on_click(move |_, window, cx| on_end(window, cx))
                .child(t!("admin.active").to_string())
        });
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
        let bar = if self.selected_count == 0 {
            bar
        } else {
            bar.child(t!("status.selected", count = self.selected_count).to_string())
        };
        bar.children(admin)
    }
}
