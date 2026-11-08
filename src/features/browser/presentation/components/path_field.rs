use gpui::{App, Entity, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT};
use crate::core::theme::metrics::{
    CONTROL_RADIUS, CRUMBS_HEIGHT, CRUMBS_PAD_LEFT, CRUMBS_PAD_RIGHT,
};
use crate::global_components::text_field::text_field_view::TextField;

#[derive(IntoElement)]
pub struct PathField {
    pub field: Entity<TextField>,
}

impl RenderOnce for PathField {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_1()
            .min_w_0()
            .items_center()
            .h(px(CRUMBS_HEIGHT))
            .pl(px(CRUMBS_PAD_LEFT))
            .pr(px(CRUMBS_PAD_RIGHT))
            .rounded(px(CONTROL_RADIUS))
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(ACCENT))
            .text_color(rgb(FG_BRIGHT))
            .child(self.field)
    }
}
