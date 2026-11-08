use std::path::PathBuf;

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::preview::domain::entities::markdown_block::MarkdownBlock;
use crate::features::preview::domain::entities::text_line::TextLine;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PreviewContent {
    Image {
        path: PathBuf,
        dimensions: Option<(usize, usize)>,
    },
    Text {
        lines: Vec<TextLine>,
        truncated: bool,
    },
    Markdown {
        blocks: Vec<MarkdownBlock>,
        truncated: bool,
    },
    Pdf {
        page: PathBuf,
        dimensions: Option<(usize, usize)>,
        pages: Option<usize>,
    },
    Folder {
        entries: Vec<FileEntry>,
        total: usize,
    },
    Empty,
    Unsupported,
    Failed,
}
