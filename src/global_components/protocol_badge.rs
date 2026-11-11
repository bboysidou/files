use gpui::{App, FontWeight, Hsla, IntoElement, RenderOnce, Window, div, hsla, prelude::*, px};

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::core::theme::colors::{
    BADGE_BACKGROUND_OPACITY, BADGE_LIGHTNESS, BADGE_SATURATION, DANGER, FG_NAV,
};
use crate::core::theme::metrics::{
    BADGE_PAD_Y, BADGE_RADIUS, PROTOCOL_BADGE_GAP, PROTOCOL_BADGE_PAD_X, PROTOCOL_ICON_SIZE,
    TEXT_TINY,
};
use crate::global_components::icon_glyph::IconGlyph;

const OPAQUE: f32 = 1.0;

#[derive(IntoElement)]
pub struct ProtocolBadge {
    pub protocol: NetworkProtocol,
}

impl RenderOnce for ProtocolBadge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let protocol = self.protocol;
        let color: Hsla = hsla(protocol.hue(), BADGE_SATURATION, BADGE_LIGHTNESS, OPAQUE);
        let (icon, tint) = if protocol.encrypted() {
            (Icon::Key, FG_NAV)
        } else {
            (Icon::Warning, DANGER)
        };
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(PROTOCOL_BADGE_GAP))
            .child(IconGlyph::new(icon, tint).size(PROTOCOL_ICON_SIZE))
            .child(
                div()
                    .px(px(PROTOCOL_BADGE_PAD_X))
                    .py(px(BADGE_PAD_Y))
                    .rounded(px(BADGE_RADIUS))
                    .bg(color.opacity(BADGE_BACKGROUND_OPACITY))
                    .text_color(color)
                    .text_size(px(TEXT_TINY))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(protocol.badge()),
            )
    }
}
