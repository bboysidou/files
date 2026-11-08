use std::fs::{self, DirEntry, Metadata};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::core::constants::files::{HIDDEN_PREFIX, TRASH_FILES_DIR};
use crate::core::enums::files::file_kind::FileKind;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::list_error::ListError;
use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::places::domain::entities::place::PlaceTarget;

pub struct LocalDirectoryRepository;

impl LocalDirectoryRepository {
    fn entry(item: DirEntry) -> FileEntry {
        Self::entry_at(item.path())
    }

    fn entry_at(path: PathBuf) -> FileEntry {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());
        let metadata = fs::metadata(&path)
            .or_else(|_| fs::symlink_metadata(&path))
            .ok();
        let is_dir = metadata.as_ref().is_some_and(Metadata::is_dir);
        let extension = (!is_dir)
            .then(|| {
                path.extension()
                    .map(|extension| extension.to_string_lossy().into_owned())
            })
            .flatten();
        FileEntry {
            kind: FileKind::classify(extension.as_deref(), is_dir),
            hidden: name.starts_with(HIDDEN_PREFIX),
            size: metadata.as_ref().filter(|_| !is_dir).map(Metadata::len),
            child_count: is_dir.then(|| Self::count_children(&path)).flatten(),
            modified: metadata
                .as_ref()
                .and_then(|metadata| metadata.modified().ok()),
            name,
            path,
            extension,
            is_dir,
        }
    }

    fn count_children(directory: &Path) -> Option<usize> {
        fs::read_dir(directory).ok().map(Iterator::count)
    }

    fn error_of(kind: ErrorKind) -> ListError {
        match kind {
            ErrorKind::NotFound => ListError::NotFound,
            ErrorKind::PermissionDenied => ListError::PermissionDenied,
            _ => ListError::Failed,
        }
    }
}

impl DirectoryRepository for LocalDirectoryRepository {
    fn list(&self, directory: &Path) -> Result<Vec<FileEntry>, ListError> {
        let items = fs::read_dir(directory).map_err(|error| Self::error_of(error.kind()))?;
        Ok(items.filter_map(Result::ok).map(Self::entry).collect())
    }

    fn entry(&self, path: &Path) -> Option<FileEntry> {
        fs::symlink_metadata(path).ok()?;
        Some(Self::entry_at(path.to_path_buf()))
    }

    fn resolve(&self, target: &PlaceTarget) -> Option<PathBuf> {
        match target {
            PlaceTarget::Directory(path) => Some(path.clone()),
            PlaceTarget::Trash => dirs::data_dir().map(|data| data.join(TRASH_FILES_DIR)),
            PlaceTarget::Overview => None,
        }
    }
}
