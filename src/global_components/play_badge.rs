use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgba};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{FG, SCRIM};
use crate::global_components::icon_glyph::IconGlyph;

const NUDGE_RATIO: f32 = 0.1;

#[derive(IntoElement)]
pub struct PlayBadge {
    pub size: f32,
    pub icon_size: f32,
}

impl RenderOnce for PlayBadge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .absolute()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .size(px(self.size))
                    .rounded_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .pl(px(self.icon_size * NUDGE_RATIO))
                    .bg(rgba(SCRIM))
                    .child(IconGlyph::new(Icon::Play, FG).size(self.icon_size)),
            )
    }
}
