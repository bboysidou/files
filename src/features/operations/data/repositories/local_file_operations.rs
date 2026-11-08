use std::fs;
use std::io::{self, ErrorKind};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use trash::TrashItem;
use trash::os_limited;

use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::repositories::file_operations::FileOperations;

pub struct LocalFileOperations;

impl LocalFileOperations {
    fn error_of(error: &io::Error) -> OperationError {
        match error.kind() {
            ErrorKind::NotFound => OperationError::NotFound,
            ErrorKind::PermissionDenied => OperationError::PermissionDenied,
            ErrorKind::AlreadyExists => OperationError::AlreadyExists,
            _ => OperationError::Failed,
        }
    }

    fn copy_tree(from: &Path, to: &Path) -> io::Result<()> {
        let metadata = fs::symlink_metadata(from)?;
        if metadata.file_type().is_symlink() {
            return symlink(fs::read_link(from)?, to);
        }
        if !metadata.is_dir() {
            return fs::copy(from, to).map(|_| ());
        }
        fs::create_dir(to)?;
        for item in fs::read_dir(from)? {
            let item = item?;
            Self::copy_tree(&item.path(), &to.join(item.file_name()))?;
        }
        Ok(())
    }

    fn remove_tree(path: &Path) -> io::Result<()> {
        let metadata = fs::symlink_metadata(path)?;
        if metadata.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }
    }

    fn newest_item_for(items: &[TrashItem], path: &Path) -> Option<TrashItem> {
        items
            .iter()
            .filter(|item| item.original_path() == path)
            .max_by_key(|item| item.time_deleted)
            .cloned()
    }

    fn ensure_free(&self, path: &Path) -> Result<(), OperationError> {
        if self.exists(path) {
            Err(OperationError::AlreadyExists)
        } else {
            Ok(())
        }
    }
}

impl FileOperations for LocalFileOperations {
    fn exists(&self, path: &Path) -> bool {
        fs::symlink_metadata(path).is_ok()
    }

    fn copy(&self, from: &Path, to: &Path) -> Result<(), OperationError> {
        self.ensure_free(to)?;
        Self::copy_tree(from, to).map_err(|error| Self::error_of(&error))
    }

    fn move_path(&self, from: &Path, to: &Path) -> Result<(), OperationError> {
        self.ensure_free(to)?;
        match fs::rename(from, to) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::CrossesDevices => Self::copy_tree(from, to)
                .and_then(|()| Self::remove_tree(from))
                .map_err(|error| Self::error_of(&error)),
            Err(error) => Err(Self::error_of(&error)),
        }
    }

    fn trash(&self, paths: &[PathBuf]) -> Result<(), OperationError> {
        trash::delete_all(paths).map_err(|_| OperationError::Failed)
    }

    fn restore(&self, paths: &[PathBuf]) -> Result<(), OperationError> {
        let items = os_limited::list().map_err(|_| OperationError::Failed)?;
        let wanted: Vec<TrashItem> = paths
            .iter()
            .map(|path| Self::newest_item_for(&items, path).ok_or(OperationError::NotFound))
            .collect::<Result<_, _>>()?;
        os_limited::restore_all(wanted).map_err(|_| OperationError::AlreadyExists)
    }

    fn create_folder(&self, path: &Path) -> Result<(), OperationError> {
        fs::create_dir(path).map_err(|error| Self::error_of(&error))
    }

    fn remove_folder(&self, path: &Path) -> Result<(), OperationError> {
        match fs::remove_dir(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::DirectoryNotEmpty => {
                self.trash(&[path.to_path_buf()])
            }
            Err(error) => Err(Self::error_of(&error)),
        }
    }
}
