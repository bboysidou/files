use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::tags::tag_color::TagColor;
use crate::core::theme::metrics::{NAV_PAD_X, RAIL_ITEM_COLLAPSED, TAG_DOT, TAG_GAP, TAG_PAD_Y};

#[derive(IntoElement)]
pub struct RailTags {
    pub open: bool,
}

impl RenderOnce for RailTags {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let dots = TagColor::ALL.iter().enumerate().map(|(index, tag)| {
            div()
                .id(("tag", index))
                .size(px(TAG_DOT))
                .flex_shrink_0()
                .rounded_full()
                .bg(rgb(tag.hex()))
                .cursor_pointer()
        });
        let row = div()
            .flex()
            .flex_shrink_0()
            .gap(px(TAG_GAP))
            .py(px(TAG_PAD_Y));
        let row = if self.open {
            row.flex_wrap().px(px(NAV_PAD_X))
        } else {
            row.flex_col().pl(px((RAIL_ITEM_COLLAPSED - TAG_DOT) / 2.0))
        };
        row.children(dots)
    }
}
