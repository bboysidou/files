use std::path::{Path, PathBuf};

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::list_error::ListError;
use crate::features::places::domain::entities::place::PlaceTarget;

pub trait DirectoryRepository: Send + Sync {
    fn list(&self, directory: &Path) -> Result<Vec<FileEntry>, ListError>;
    fn resolve(&self, target: &PlaceTarget) -> Option<PathBuf>;
}
