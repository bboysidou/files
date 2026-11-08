use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{FG, FG_DIM};
use crate::core::theme::metrics::{PREVIEW_LINE_GAP, PREVIEW_LINE_HEIGHT, PREVIEW_NUMBER_WIDTH};

#[derive(IntoElement)]
pub struct PreviewLine {
    pub number: usize,
    pub text: SharedString,
}

impl RenderOnce for PreviewLine {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .gap(px(PREVIEW_LINE_GAP))
            .h(px(PREVIEW_LINE_HEIGHT))
            .whitespace_nowrap()
            .child(
                div()
                    .flex()
                    .flex_shrink_0()
                    .justify_end()
                    .w(px(PREVIEW_NUMBER_WIDTH))
                    .text_color(rgb(FG_DIM))
                    .child(self.number.to_string()),
            )
            .child(div().text_color(rgb(FG)).child(self.text))
    }
}
