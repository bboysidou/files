use gpui::{App, IntoElement, RenderOnce, StyledText, Window, div, prelude::*, px, rgb};

use crate::core::helpers::format::rich_highlights::color_highlight;
use crate::core::theme::colors::{FG, FG_DIM};
use crate::core::theme::metrics::{PREVIEW_LINE_GAP, PREVIEW_LINE_HEIGHT, PREVIEW_NUMBER_WIDTH};
use crate::features::preview::domain::entities::text_line::TextLine;

#[derive(IntoElement)]
pub struct PreviewLine {
    pub number: usize,
    pub line: TextLine,
}

impl RenderOnce for PreviewLine {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let highlights = self
            .line
            .runs
            .into_iter()
            .map(|run| color_highlight(run.range, run.color));
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
            .child(
                div()
                    .text_color(rgb(FG))
                    .child(StyledText::new(self.line.text).with_highlights(highlights)),
            )
    }
}
