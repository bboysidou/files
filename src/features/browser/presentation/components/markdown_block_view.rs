use gpui::{
    AnyElement, App, FontWeight, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb,
};

use crate::core::theme::colors::{ACCENT, CARD_SUNKEN, DIVIDER, FG, FG_BRIGHT, FG_MUTED, LINE};
use crate::core::theme::metrics::{
    MARKDOWN_CODE_PAD, MARKDOWN_CODE_RADIUS, MARKDOWN_HEADING_SIZES, MARKDOWN_INDENT,
    MARKDOWN_MARKER_WIDTH, MARKDOWN_QUOTE_BAR, MARKDOWN_QUOTE_PAD, MARKDOWN_RULE,
};
use crate::features::browser::presentation::components::rich_text_view::RichTextView;
use crate::features::preview::domain::entities::markdown_block::{MarkdownBlock, RichText};

#[derive(IntoElement)]
pub struct MarkdownBlockView {
    pub block: MarkdownBlock,
}

fn heading(level: u8, text: RichText) -> AnyElement {
    let size = MARKDOWN_HEADING_SIZES[usize::from(level.clamp(1, 6)) - 1];
    div()
        .text_size(px(size))
        .font_weight(FontWeight::BOLD)
        .text_color(rgb(FG_BRIGHT))
        .child(RichTextView { text })
        .into_any_element()
}

fn item(depth: usize, marker: String, text: RichText) -> AnyElement {
    div()
        .flex()
        .pl(px(MARKDOWN_INDENT * depth as f32))
        .child(
            div()
                .flex_shrink_0()
                .w(px(MARKDOWN_MARKER_WIDTH))
                .text_color(rgb(ACCENT))
                .child(marker),
        )
        .child(div().flex_1().min_w_0().child(RichTextView { text }))
        .into_any_element()
}

fn quote(text: RichText) -> AnyElement {
    div()
        .border_l(px(MARKDOWN_QUOTE_BAR))
        .border_color(rgb(LINE))
        .pl(px(MARKDOWN_QUOTE_PAD))
        .text_color(rgb(FG_MUTED))
        .child(RichTextView { text })
        .into_any_element()
}

fn code(lines: Vec<String>) -> AnyElement {
    div()
        .p(px(MARKDOWN_CODE_PAD))
        .rounded(px(MARKDOWN_CODE_RADIUS))
        .bg(rgb(CARD_SUNKEN))
        .overflow_hidden()
        .whitespace_nowrap()
        .text_color(rgb(FG))
        .children(lines.into_iter().map(|line| div().child(line)))
        .into_any_element()
}

impl RenderOnce for MarkdownBlockView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        match self.block {
            MarkdownBlock::Heading { level, text } => heading(level, text),
            MarkdownBlock::Paragraph(text) => div().child(RichTextView { text }).into_any_element(),
            MarkdownBlock::Item {
                depth,
                marker,
                text,
            } => item(depth, marker, text),
            MarkdownBlock::Quote(text) => quote(text),
            MarkdownBlock::Code(lines) => code(lines),
            MarkdownBlock::Rule => div()
                .h(px(MARKDOWN_RULE))
                .bg(rgb(DIVIDER))
                .into_any_element(),
        }
    }
}
