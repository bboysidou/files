use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT, FG_MUTED, FG_SOFT, LINE_STRONG};
use crate::core::theme::metrics::{
    ICON_SIZE_SMALL, TAB_CLOSE_RADIUS, TAB_CLOSE_SIZE, TAB_GAP, TAB_HEIGHT, TAB_INDICATOR,
    TAB_PAD_LEFT, TAB_PAD_RIGHT, TAB_RADIUS, TEXT_TINY,
};
use crate::global_components::icon_glyph::IconGlyph;

const CLOSE_GROUP: &str = "tab-close";

#[derive(IntoElement)]
pub struct SheetTab {
    pub index: usize,
    pub title: SharedString,
    pub active: bool,
}

impl RenderOnce for SheetTab {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let close = div()
            .id(("tab-close", self.index))
            .group(CLOSE_GROUP)
            .size(px(TAB_CLOSE_SIZE))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(TAB_CLOSE_RADIUS))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(LINE_STRONG)))
            .child(
                IconGlyph::new(Icon::Close, FG_MUTED)
                    .size(ICON_SIZE_SMALL)
                    .group_hover(CLOSE_GROUP, FG_BRIGHT),
            );
        let number = div()
            .text_size(px(TEXT_TINY))
            .text_color(rgb(FG_MUTED))
            .child((self.index + 1).to_string());
        let tab = div()
            .id(("tab", self.index))
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(TAB_GAP))
            .h(px(TAB_HEIGHT))
            .pl(px(TAB_PAD_LEFT))
            .pr(px(TAB_PAD_RIGHT))
            .rounded_tl(px(TAB_RADIUS))
            .rounded_tr(px(TAB_RADIUS))
            .whitespace_nowrap()
            .cursor_pointer();
        let tab = if self.active {
            tab.bg(rgb(CARD))
                .text_color(rgb(FG_BRIGHT))
                .border_b(px(TAB_INDICATOR))
                .border_color(rgb(ACCENT))
        } else {
            tab.text_color(rgb(FG_SOFT))
        };
        tab.child(self.title).child(number).child(close)
    }
}
