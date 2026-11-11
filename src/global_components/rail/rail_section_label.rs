use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::theme::colors::{FG_MUTED, LINE_STRONG};
use crate::core::theme::metrics::{
    NAV_PAD_X, RAIL_DIVIDER_MARGIN, RAIL_DIVIDER_WIDTH, RAIL_ITEM_COLLAPSED,
    RAIL_SECTION_GAP_BOTTOM, RAIL_SECTION_GAP_TOP, TEXT_TINY,
};
use crate::global_components::rail::rail_label::RailLabel;
use crate::global_components::rail::rail_motion::RailMotion;

#[derive(IntoElement)]
pub struct RailSectionLabel {
    key: &'static str,
    motion: RailMotion,
}

impl RailSectionLabel {
    pub fn new(key: &'static str, motion: RailMotion) -> Self {
        Self { key, motion }
    }
}

impl RenderOnce for RailSectionLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if !self.motion.open {
            return div()
                .flex_shrink_0()
                .h(px(1.0))
                .w(px(RAIL_DIVIDER_WIDTH))
                .my(px(RAIL_DIVIDER_MARGIN))
                .ml(px((RAIL_ITEM_COLLAPSED - RAIL_DIVIDER_WIDTH) / 2.0))
                .bg(rgb(LINE_STRONG));
        }
        div()
            .flex()
            .flex_shrink_0()
            .px(px(NAV_PAD_X))
            .mt(px(RAIL_SECTION_GAP_TOP))
            .mb(px(RAIL_SECTION_GAP_BOTTOM))
            .text_size(px(TEXT_TINY))
            .text_color(rgb(FG_MUTED))
            .child(RailLabel::new(
                self.key,
                t!(self.key).to_string(),
                self.motion,
            ))
    }
}
