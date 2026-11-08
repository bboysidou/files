use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::core::constants::preview::{
    MARKDOWN_BULLET, MARKDOWN_CELL_SEPARATOR, MARKDOWN_TASK_DONE, MARKDOWN_TASK_OPEN,
};
use crate::core::enums::preview::inline_style::InlineStyle;
use crate::features::preview::domain::entities::markdown_block::{
    InlineRun, MarkdownBlock, RichText,
};

#[derive(Default)]
struct Builder {
    blocks: Vec<MarkdownBlock>,
    text: RichText,
    styles: Vec<InlineStyle>,
    lists: Vec<Option<u64>>,
    marker: Option<String>,
    quote: usize,
    code: Option<Vec<String>>,
    heading: Option<u8>,
}

impl Builder {
    fn push_text(&mut self, piece: &str, extra: Option<InlineStyle>) {
        if let Some(code) = self.code.as_mut() {
            code.extend(piece.lines().map(str::to_string));
            return;
        }
        let start = self.text.text.len();
        self.text.text.push_str(piece);
        let end = self.text.text.len();
        for style in self.styles.iter().copied().chain(extra) {
            self.text.runs.push(InlineRun {
                range: start..end,
                style,
            });
        }
    }

    fn flush(&mut self) {
        let text = std::mem::take(&mut self.text);
        if text.text.trim().is_empty() {
            return;
        }
        let block = if let Some(level) = self.heading.take() {
            MarkdownBlock::Heading { level, text }
        } else if let Some(marker) = self.marker.take() {
            MarkdownBlock::Item {
                depth: self.lists.len().saturating_sub(1),
                marker,
                text,
            }
        } else if self.quote > 0 {
            MarkdownBlock::Quote(text)
        } else {
            MarkdownBlock::Paragraph(text)
        };
        self.blocks.push(block);
    }

    fn next_marker(&mut self) -> String {
        match self.lists.last_mut() {
            Some(Some(number)) => {
                *number += 1;
                format!("{}.", *number - 1)
            }
            _ => MARKDOWN_BULLET.to_string(),
        }
    }

    fn start(&mut self, tag: Tag) {
        match tag {
            Tag::Heading { level, .. } => self.heading = Some(heading_level(level)),
            Tag::List(first) => {
                self.flush();
                self.lists.push(first);
            }
            Tag::Item => {
                self.flush();
                self.marker = Some(self.next_marker());
            }
            Tag::BlockQuote(_) => self.quote += 1,
            Tag::CodeBlock(_) => self.code = Some(Vec::new()),
            Tag::TableCell if !self.text.text.is_empty() => {
                self.push_text(MARKDOWN_CELL_SEPARATOR, None)
            }
            Tag::Strong => self.styles.push(InlineStyle::Strong),
            Tag::Emphasis => self.styles.push(InlineStyle::Emphasis),
            Tag::Link { .. } => self.styles.push(InlineStyle::Link),
            _ => {}
        }
    }

    fn end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Heading(_)
            | TagEnd::Paragraph
            | TagEnd::Item
            | TagEnd::TableRow
            | TagEnd::TableHead => self.flush(),
            TagEnd::List(_) => {
                self.flush();
                self.lists.pop();
            }
            TagEnd::BlockQuote(_) => {
                self.flush();
                self.quote = self.quote.saturating_sub(1);
            }
            TagEnd::CodeBlock => {
                let lines = self.code.take().unwrap_or_default();
                self.blocks.push(MarkdownBlock::Code(lines));
            }
            TagEnd::Strong | TagEnd::Emphasis | TagEnd::Link => {
                self.styles.pop();
            }
            _ => {}
        }
    }

    fn event(&mut self, event: Event) {
        match event {
            Event::Start(tag) => self.start(tag),
            Event::End(tag) => self.end(tag),
            Event::Text(text) => self.push_text(&text, None),
            Event::Code(text) => self.push_text(&text, Some(InlineStyle::Code)),
            Event::SoftBreak | Event::HardBreak => self.push_text(" ", None),
            Event::Rule => {
                self.flush();
                self.blocks.push(MarkdownBlock::Rule);
            }
            Event::TaskListMarker(done) => {
                let marker = if done {
                    MARKDOWN_TASK_DONE
                } else {
                    MARKDOWN_TASK_OPEN
                };
                self.marker = Some(marker.to_string());
            }
            _ => {}
        }
    }
}

fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

pub fn parse_markdown(source: &str) -> Vec<MarkdownBlock> {
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH;
    let mut builder = Builder::default();
    for event in Parser::new_ext(source, options) {
        builder.event(event);
    }
    builder.flush();
    builder.blocks
}
