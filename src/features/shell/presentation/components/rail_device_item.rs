use gpui::{
    App, FontWeight, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb,
};

use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::theme::focus_ring::focus_ring;
use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT, FG_NAV, ON_ACCENT};
use crate::core::theme::metrics::{
    CONTROL_RADIUS, NAV_GAP, NAV_HEIGHT, NAV_PAD_X, RAIL_ITEM_COLLAPSED,
};
use crate::features::shell::presentation::components::rail_label::RailLabel;
use crate::features::shell::presentation::state::rail_motion::RailMotion;
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::icon_button::IconButton;
use crate::global_components::icon_glyph::IconGlyph;

const GROUP: &str = "rail-device";

#[derive(IntoElement)]
pub struct RailDeviceItem {
    pub index: usize,
    pub id: &'static str,
    pub icon: Icon,
    pub label: SharedString,
    pub mounted: bool,
    pub active: bool,
    pub cursor: bool,
    pub motion: RailMotion,
    pub on_select: IndexHandler,
    pub on_eject: ClickHandler,
}

impl RenderOnce for RailDeviceItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (index, on_select) = (self.index, self.on_select);
        let color = if self.active { ON_ACCENT } else { FG_NAV };
        let item = div()
            .id((self.id, index))
            .group(GROUP)
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(NAV_GAP))
            .px(px(NAV_PAD_X))
            .rounded(px(CONTROL_RADIUS))
            .border_1()
            .border_color(focus_ring(self.cursor))
            .cursor_pointer()
            .text_color(rgb(color))
            .on_click(move |_, window, cx| on_select(index, window, cx));
        let item = if self.active {
            item.bg(rgb(ACCENT)).font_weight(FontWeight::SEMIBOLD)
        } else {
            item.hover(|style| style.bg(rgb(CARD)).text_color(rgb(FG_BRIGHT)))
        };
        let glyph = IconGlyph::new(self.icon, color)
            .group_hover(GROUP, if self.active { ON_ACCENT } else { FG_BRIGHT });
        if !self.motion.open {
            return item.size(px(RAIL_ITEM_COLLAPSED)).child(glyph);
        }
        item.w_full()
            .h(px(NAV_HEIGHT))
            .child(glyph)
            .child(RailLabel::new("device-label", self.label, self.motion))
            .when(self.mounted, |item| {
                item.child(
                    IconButton::new(("eject", index), Icon::Eject)
                        .radius(CONTROL_RADIUS)
                        .on_click(self.on_eject),
                )
            })
    }
}
