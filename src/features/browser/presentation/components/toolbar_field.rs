use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT, FG_MUTED};
use crate::core::theme::metrics::{
    CONTROL_RADIUS, CRUMBS_HEIGHT, FILTER_FIELD_GAP, FILTER_FIELD_PAD_X, FILTER_FIELD_WIDTH,
    TEXT_TINY,
};
use crate::features::browser::presentation::state::filter_model::FilterModel;
use crate::global_components::icon_glyph::IconGlyph;

#[derive(IntoElement)]
pub struct ToolbarField {
    pub icon: Icon,
    pub model: FilterModel,
}

impl RenderOnce for ToolbarField {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(FILTER_FIELD_GAP))
            .w(px(FILTER_FIELD_WIDTH))
            .h(px(CRUMBS_HEIGHT))
            .px(px(FILTER_FIELD_PAD_X))
            .rounded(px(CONTROL_RADIUS))
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(ACCENT))
            .text_color(rgb(FG_BRIGHT))
            .child(IconGlyph::new(self.icon, ACCENT))
            .child(self.model.field)
            .child(
                div()
                    .flex_shrink_0()
                    .text_size(px(TEXT_TINY))
                    .text_color(rgb(FG_MUTED))
                    .child(self.model.count),
            )
    }
}
