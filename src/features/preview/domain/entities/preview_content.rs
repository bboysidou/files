use std::path::PathBuf;

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::preview::domain::entities::archive_member::ArchiveMember;
use crate::features::preview::domain::entities::exif_fact::ExifFact;
use crate::features::preview::domain::entities::markdown_block::MarkdownBlock;
use crate::features::preview::domain::entities::media_facts::MediaFacts;
use crate::features::preview::domain::entities::text_line::TextLine;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PreviewContent {
    Image {
        path: PathBuf,
        dimensions: Option<(usize, usize)>,
        exif: Vec<ExifFact>,
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
    Archive {
        members: Vec<ArchiveMember>,
        total: usize,
        capped: bool,
    },
    Media {
        frame: Option<PathBuf>,
        facts: MediaFacts,
    },
    Font {
        specimen: PathBuf,
        family: Option<String>,
    },
    Folder {
        entries: Vec<FileEntry>,
        total: usize,
    },
    Empty,
    Unsupported,
    Failed,
}
