use std::path::PathBuf;

use crate::features::browser::domain::entities::file_entry::FileEntry;

#[derive(Clone, Default)]
pub struct ColumnState {
    pub parent_of: Option<PathBuf>,
    pub parent: Vec<FileEntry>,
    pub child_of: Option<PathBuf>,
    pub child: Vec<FileEntry>,
}
