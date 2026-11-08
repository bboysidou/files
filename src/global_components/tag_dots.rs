use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::tags::tag_color::TagColor;
use crate::core::theme::metrics::{ENTRY_TAG_DOT, ENTRY_TAG_GAP};

#[derive(IntoElement)]
pub struct TagDots {
    pub tags: Vec<TagColor>,
}

impl RenderOnce for TagDots {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(ENTRY_TAG_GAP))
            .children(self.tags.into_iter().map(|tag| {
                div()
                    .size(px(ENTRY_TAG_DOT))
                    .rounded_full()
                    .bg(rgb(tag.hex()))
            }))
    }
}
