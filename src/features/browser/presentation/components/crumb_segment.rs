use gpui::{
    App, FontWeight, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb,
};

use crate::core::theme::colors::{FG_BRIGHT, FG_MUTED, LINE_STRONG};
use crate::core::theme::metrics::{CRUMB_PAD_X, CRUMB_PAD_Y, CRUMB_RADIUS};
use crate::global_components::handlers::IndexHandler;

#[derive(IntoElement)]
pub struct CrumbSegment {
    pub index: usize,
    pub text: SharedString,
    pub last: bool,
    pub on_click: IndexHandler,
}

impl RenderOnce for CrumbSegment {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (index, on_click) = (self.index, self.on_click);
        let segment = div()
            .id(("crumb", index))
            .flex_shrink_0()
            .px(px(CRUMB_PAD_X))
            .py(px(CRUMB_PAD_Y))
            .rounded(px(CRUMB_RADIUS))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(LINE_STRONG)).text_color(rgb(FG_BRIGHT)))
            .on_click(move |_, window, cx| on_click(index, window, cx))
            .child(self.text);
        if self.last {
            segment
                .text_color(rgb(FG_BRIGHT))
                .font_weight(FontWeight::SEMIBOLD)
        } else {
            segment.text_color(rgb(FG_MUTED))
        }
    }
}
