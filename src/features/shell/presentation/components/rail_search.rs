use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{CARD_SUNKEN, FG_MUTED, LINE};
use crate::core::theme::metrics::{
    RAIL_ITEM_COLLAPSED, SEARCH_GAP, SEARCH_HEIGHT, SEARCH_PAD_LEFT, SEARCH_PAD_RIGHT, TEXT_SMALL,
};
use crate::features::shell::presentation::components::rail_label::RailLabel;
use crate::features::shell::presentation::state::rail_motion::RailMotion;
use crate::global_components::icon_glyph::IconGlyph;
use crate::global_components::kbd_hint::KbdHint;

const SEARCH_KEY: &str = "f";

#[derive(IntoElement)]
pub struct RailSearch {
    pub motion: RailMotion,
}

impl RenderOnce for RailSearch {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let field = div()
            .id("rail-search")
            .flex()
            .flex_shrink_0()
            .items_center()
            .pl(px(SEARCH_PAD_LEFT))
            .rounded_full()
            .bg(rgb(CARD_SUNKEN))
            .border_1()
            .border_color(rgb(LINE))
            .text_color(rgb(FG_MUTED))
            .cursor_text()
            .child(IconGlyph::new(Icon::Search, FG_MUTED));
        if !self.motion.open {
            return field.size(px(RAIL_ITEM_COLLAPSED));
        }
        let label = div()
            .flex()
            .flex_1()
            .min_w_0()
            .text_size(px(TEXT_SMALL))
            .child(RailLabel::new(
                "search-label",
                t!("rail.search").to_string(),
                self.motion,
            ));
        field
            .h(px(SEARCH_HEIGHT))
            .w_full()
            .gap(px(SEARCH_GAP))
            .pr(px(SEARCH_PAD_RIGHT))
            .child(label)
            .child(KbdHint::new(SEARCH_KEY))
    }
}
