use gpui::{
    App, FontWeight, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb, rgba,
};

use crate::core::theme::colors::BADGE_ALPHA;
use crate::core::theme::metrics::{BADGE_PAD_Y, BADGE_RADIUS, BADGE_WIDTH, TEXT_TINY};

#[derive(IntoElement)]
pub struct FileBadge {
    pub text: SharedString,
    pub color: u32,
}

impl RenderOnce for FileBadge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .justify_center()
            .w(px(BADGE_WIDTH))
            .py(px(BADGE_PAD_Y))
            .rounded(px(BADGE_RADIUS))
            .bg(rgba((self.color << 8) | BADGE_ALPHA))
            .text_color(rgb(self.color))
            .text_size(px(TEXT_TINY))
            .font_weight(FontWeight::SEMIBOLD)
            .child(self.text)
    }
}
