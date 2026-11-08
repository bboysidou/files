use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, relative, rgb};

use crate::core::theme::colors::FG;
use crate::core::theme::metrics::{MARKDOWN_BLOCK_GAP, MARKDOWN_LINE_HEIGHT, QUICK_LOOK_STAGE_PAD};
use crate::features::browser::presentation::components::markdown_block_view::MarkdownBlockView;
use crate::features::preview::domain::entities::markdown_block::MarkdownBlock;

#[derive(IntoElement)]
pub struct PreviewMarkdown {
    pub id: &'static str,
    pub blocks: Vec<MarkdownBlock>,
}

impl RenderOnce for PreviewMarkdown {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .size_full()
            .overflow_y_scroll()
            .p(px(QUICK_LOOK_STAGE_PAD))
            .flex()
            .flex_col()
            .gap(px(MARKDOWN_BLOCK_GAP))
            .line_height(relative(MARKDOWN_LINE_HEIGHT))
            .text_color(rgb(FG))
            .children(
                self.blocks
                    .into_iter()
                    .map(|block| MarkdownBlockView { block }),
            )
    }
}
