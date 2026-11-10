use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};

use trash::TrashItem;
use trash::os_limited;

use crate::core::constants::files::COPY_BUFFER_BYTES;
use crate::core::enums::operations::job_phase::JobPhase;
use crate::core::helpers::files::has_no_trash::has_no_trash;
use crate::core::helpers::files::same_device::same_device;
use crate::features::operations::data::repositories::writeback::Writeback;
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::repositories::file_operations::FileOperations;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub struct LocalFileOperations;

impl LocalFileOperations {
    fn error_of(error: &io::Error) -> OperationError {
        match error.kind() {
            ErrorKind::NotFound => OperationError::NotFound,
            ErrorKind::PermissionDenied => OperationError::PermissionDenied,
            ErrorKind::AlreadyExists => OperationError::AlreadyExists,
            ErrorKind::Interrupted => OperationError::Cancelled,
            _ => OperationError::Failed,
        }
    }

    fn copy_file(
        from: &Path,
        to: &Path,
        sink: &dyn ProgressSink,
        writeback: &mut Writeback,
    ) -> io::Result<()> {
        let mut source = File::open(from)?;
        let mut target = OpenOptions::new().write(true).create_new(true).open(to)?;
        let mut buffer = vec![0_u8; COPY_BUFFER_BYTES];
        loop {
            if sink.is_cancelled() {
                drop(target);
                let _ = fs::remove_file(to);
                return Err(io::Error::from(ErrorKind::Interrupted));
            }
            let read = source.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            target.write_all(&buffer[..read])?;
            writeback.wrote(read as u64, sink)?;
        }
        fs::set_permissions(to, fs::metadata(from)?.permissions())
    }

    fn copy_tree(
        from: &Path,
        to: &Path,
        sink: &dyn ProgressSink,
        writeback: &mut Writeback,
    ) -> io::Result<()> {
        let metadata = fs::symlink_metadata(from)?;
        if metadata.file_type().is_symlink() {
            return symlink(fs::read_link(from)?, to);
        }
        if !metadata.is_dir() {
            return Self::copy_file(from, to, sink, writeback);
        }
        fs::create_dir(to)?;
        for item in fs::read_dir(from)? {
            let item = item?;
            Self::copy_tree(&item.path(), &to.join(item.file_name()), sink, writeback)?;
        }
        Ok(())
    }

    fn copy_flushed(from: &Path, to: &Path, sink: &dyn ProgressSink) -> io::Result<()> {
        let mut writeback = Writeback::for_destination(to);
        sink.set_phase(JobPhase::Working);
        let copied = Self::copy_tree(from, to, sink, &mut writeback);
        sink.set_phase(JobPhase::Flushing);
        let flushed = writeback.flush(sink);
        sink.set_phase(JobPhase::Working);
        copied.and(flushed)
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

    fn is_dir(&self, path: &Path) -> bool {
        fs::symlink_metadata(path).is_ok_and(|meta| meta.is_dir())
    }

    fn children(&self, path: &Path) -> Vec<PathBuf> {
        fs::read_dir(path)
            .map(|items| items.flatten().map(|item| item.path()).collect())
            .unwrap_or_default()
    }

    fn copy(&self, from: &Path, to: &Path, sink: &dyn ProgressSink) -> Result<(), OperationError> {
        self.ensure_free(to)?;
        Self::copy_flushed(from, to, sink).map_err(|error| Self::error_of(&error))
    }

    fn transfer_bytes(&self, from: &Path, to: &Path) -> u64 {
        let folder = to.parent().unwrap_or(to);
        if same_device(from, folder) {
            0
        } else {
            self.size_of(from)
        }
    }

    fn size_of(&self, path: &Path) -> u64 {
        let Ok(metadata) = fs::symlink_metadata(path) else {
            return 0;
        };
        if !metadata.is_dir() {
            return metadata.len();
        }
        fs::read_dir(path)
            .map(|items| {
                items
                    .filter_map(Result::ok)
                    .map(|item| self.size_of(&item.path()))
                    .sum()
            })
            .unwrap_or_default()
    }

    fn move_path(
        &self,
        from: &Path,
        to: &Path,
        sink: &dyn ProgressSink,
    ) -> Result<(), OperationError> {
        self.ensure_free(to)?;
        match fs::rename(from, to) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::CrossesDevices => {
                Self::copy_flushed(from, to, sink).map_err(|error| Self::error_of(&error))?;
                if self.size_of(from) != self.size_of(to) {
                    return Err(OperationError::Failed);
                }
                Self::remove_tree(from).map_err(|error| Self::error_of(&error))
            }
            Err(error) => Err(Self::error_of(&error)),
        }
    }

    fn trash(&self, paths: &[PathBuf]) -> Result<(), OperationError> {
        let (without, with): (Vec<PathBuf>, Vec<PathBuf>) =
            paths.iter().cloned().partition(|path| has_no_trash(path));
        self.delete(&without)?;
        if with.is_empty() {
            return Ok(());
        }
        trash::delete_all(&with).map_err(|_| OperationError::Failed)
    }

    fn delete(&self, paths: &[PathBuf]) -> Result<(), OperationError> {
        paths
            .iter()
            .try_for_each(|path| Self::remove_tree(path).map_err(|error| Self::error_of(&error)))
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
                if has_no_trash(path) {
                    return Err(OperationError::NotEmpty);
                }
                self.trash(&[path.to_path_buf()])
            }
            Err(error) => Err(Self::error_of(&error)),
        }
    }

    fn create_file(&self, path: &Path) -> Result<(), OperationError> {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map(drop)
            .map_err(|error| Self::error_of(&error))
    }

    fn link(&self, target: &Path, link: &Path) -> Result<(), OperationError> {
        symlink(target, link).map_err(|error| Self::error_of(&error))
    }

    fn set_mode(&self, path: &Path, mode: u32) -> Result<(), OperationError> {
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
            .map_err(|error| Self::error_of(&error))
    }
}
