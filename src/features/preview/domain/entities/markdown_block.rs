use std::ops::Range;

use crate::core::enums::preview::inline_style::InlineStyle;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InlineRun {
    pub range: Range<usize>,
    pub style: InlineStyle,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RichText {
    pub text: String,
    pub runs: Vec<InlineRun>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MarkdownBlock {
    Heading {
        level: u8,
        text: RichText,
    },
    Paragraph(RichText),
    Item {
        depth: usize,
        marker: String,
        text: RichText,
    },
    Quote(RichText),
    Code(Vec<String>),
    Rule,
}
