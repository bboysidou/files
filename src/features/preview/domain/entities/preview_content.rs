use std::path::PathBuf;

use crate::features::browser::domain::entities::file_entry::FileEntry;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PreviewContent {
    Image {
        path: PathBuf,
        dimensions: Option<(usize, usize)>,
    },
    Text {
        lines: Vec<String>,
        truncated: bool,
    },
    Folder {
        entries: Vec<FileEntry>,
        total: usize,
    },
    Empty,
    Unsupported,
    Failed,
}
