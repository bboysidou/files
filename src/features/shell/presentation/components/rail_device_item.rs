use gpui::{
    App, Div, FontWeight, Hsla, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
    rgb,
};

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::core::helpers::files::extension_color::extension_color;
use crate::core::helpers::theme::focus_ring::focus_ring;
use crate::core::theme::colors::{
    ACCENT, BADGE_BACKGROUND_OPACITY, CARD, DANGER, FG_BRIGHT, FG_NAV, ON_ACCENT,
};
use crate::core::theme::metrics::{
    BADGE_PAD_Y, BADGE_RADIUS, CONTROL_RADIUS, NAV_GAP, NAV_HEIGHT, NAV_PAD_X,
    PROTOCOL_BADGE_PAD_X, RAIL_ITEM_COLLAPSED, TEXT_TINY,
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
    pub trailing: Option<Icon>,
    pub protocol: Option<NetworkProtocol>,
    pub active: bool,
    pub cursor: bool,
    pub motion: RailMotion,
    pub on_select: IndexHandler,
    pub on_eject: ClickHandler,
    pub on_edit: Option<ClickHandler>,
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
            .when_some(self.protocol, |item, protocol| {
                item.child(protocol_badge(protocol))
            })
            .when_some(self.on_edit, |item, on_edit| {
                item.child(
                    IconButton::new(("edit", index), Icon::Edit)
                        .radius(CONTROL_RADIUS)
                        .on_click(on_edit),
                )
            })
            .when_some(self.trailing, |item, icon| {
                item.child(
                    IconButton::new(("trailing", index), icon)
                        .radius(CONTROL_RADIUS)
                        .on_click(self.on_eject),
                )
            })
    }
}

fn protocol_badge(protocol: NetworkProtocol) -> Div {
    let color: Hsla = if protocol.encrypted() {
        extension_color(Some(protocol.badge()))
    } else {
        rgb(DANGER).into()
    };
    div()
        .flex_shrink_0()
        .px(px(PROTOCOL_BADGE_PAD_X))
        .py(px(BADGE_PAD_Y))
        .rounded(px(BADGE_RADIUS))
        .bg(color.opacity(BADGE_BACKGROUND_OPACITY))
        .text_color(color)
        .text_size(px(TEXT_TINY))
        .font_weight(FontWeight::SEMIBOLD)
        .child(protocol.badge())
}
