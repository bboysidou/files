use gpui::{App, FontWeight, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{ACCENT, FG_BRIGHT, ON_ACCENT};
use crate::core::theme::metrics::{
    BRAND_COLLAPSED_PAD_BOTTOM, BRAND_GAP, BRAND_MARK, BRAND_MARK_RADIUS, BRAND_PAD_BOTTOM,
    BRAND_PAD_X, BRAND_TOGGLE_RADIUS, ICON_BUTTON_SIZE, RAIL_ITEM_COLLAPSED, TEXT_BRAND,
};
use crate::features::shell::presentation::components::rail_label::RailLabel;
use crate::features::shell::presentation::state::rail_motion::RailMotion;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_button::IconButton;
use crate::global_components::icon_glyph::IconGlyph;

#[derive(IntoElement)]
pub struct RailBrand {
    pub motion: RailMotion,
    pub on_toggle: ClickHandler,
}

impl RenderOnce for RailBrand {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let toggle = IconButton::new("rail-toggle", Icon::Sidebar)
            .radius(BRAND_TOGGLE_RADIUS)
            .on_click(self.on_toggle);
        let row = div().flex().flex_shrink_0().items_center();
        if !self.motion.open {
            return row
                .pb(px(BRAND_COLLAPSED_PAD_BOTTOM))
                .pl(px((RAIL_ITEM_COLLAPSED - ICON_BUTTON_SIZE) / 2.0))
                .child(toggle);
        }
        let mark = div()
            .size(px(BRAND_MARK))
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(BRAND_MARK_RADIUS))
            .bg(rgb(ACCENT))
            .child(IconGlyph::new(Icon::Folder, ON_ACCENT));
        let title = div()
            .flex()
            .flex_1()
            .min_w_0()
            .text_size(px(TEXT_BRAND))
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(FG_BRIGHT))
            .child(RailLabel::new(
                "brand-label",
                t!("app.name").to_string(),
                self.motion,
            ));
        row.gap(px(BRAND_GAP))
            .px(px(BRAND_PAD_X))
            .pb(px(BRAND_PAD_BOTTOM))
            .child(mark)
            .child(title)
            .child(toggle)
    }
}
