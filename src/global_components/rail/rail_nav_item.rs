use gpui::{
    App, FontWeight, IntoElement, MouseButton, RenderOnce, Window, div, prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::enums::places::place_kind::PlaceKind;
use crate::core::helpers::theme::focus_ring::focus_ring;
use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT, FG_NAV, ON_ACCENT};
use crate::core::theme::metrics::{
    CONTROL_RADIUS, NAV_GAP, NAV_HEIGHT, NAV_PAD_X, RAIL_ITEM_COLLAPSED,
};
use crate::global_components::drag::accept_paths::accept_paths;
use crate::global_components::handlers::{IndexHandler, PathsHandler, PointHandler};
use crate::global_components::icon_glyph::IconGlyph;
use crate::global_components::rail::rail_label::RailLabel;
use crate::global_components::rail::rail_motion::RailMotion;

const GROUP: &str = "rail-nav-item";

#[derive(IntoElement)]
pub struct RailNavItem {
    pub index: usize,
    pub kind: PlaceKind,
    pub active: bool,
    pub cursor: bool,
    pub motion: RailMotion,
    pub on_select: IndexHandler,
    pub on_drop: Option<PathsHandler>,
    pub on_context: Option<PointHandler>,
}

impl RenderOnce for RailNavItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_select = self.on_select;
        let item = div()
            .id(("place", self.index))
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
            .on_click(move |_, window, cx| on_select(self.index, window, cx));
        let item = item.when_some(self.on_context, |item, on_context| {
            item.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                cx.stop_propagation();
                on_context(event.position, window, cx);
            })
        });
        let item = item.when_some(self.on_drop, accept_paths);
        let item = if self.motion.open {
            item.w_full().h(px(NAV_HEIGHT))
        } else {
            item.size(px(RAIL_ITEM_COLLAPSED))
        };
        let (item, glyph) = if self.active {
            let item = item
                .bg(rgb(ACCENT))
                .text_color(rgb(ON_ACCENT))
                .font_weight(FontWeight::SEMIBOLD);
            (item, IconGlyph::new(self.kind.icon(), ON_ACCENT))
        } else {
            let item = item
                .text_color(rgb(FG_NAV))
                .hover(|style| style.bg(rgb(CARD)).text_color(rgb(FG_BRIGHT)));
            (
                item,
                IconGlyph::new(self.kind.icon(), FG_NAV).group_hover(GROUP, FG_BRIGHT),
            )
        };
        let item = item.child(glyph);
        if !self.motion.open {
            return item;
        }
        item.child(RailLabel::new(
            "nav-label",
            t!(self.kind.label_key()).to_string(),
            self.motion,
        ))
    }
}
