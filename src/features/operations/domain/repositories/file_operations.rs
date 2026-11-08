use std::path::{Path, PathBuf};

use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub trait FileOperations: Send + Sync {
    fn exists(&self, path: &Path) -> bool;
    fn is_dir(&self, path: &Path) -> bool;
    fn children(&self, path: &Path) -> Vec<PathBuf>;
    fn copy(&self, from: &Path, to: &Path, sink: &dyn ProgressSink) -> Result<(), OperationError>;
    fn move_path(
        &self,
        from: &Path,
        to: &Path,
        sink: &dyn ProgressSink,
    ) -> Result<(), OperationError>;
    fn size_of(&self, path: &Path) -> u64;
    fn trash(&self, paths: &[PathBuf]) -> Result<(), OperationError>;
    fn restore(&self, paths: &[PathBuf]) -> Result<(), OperationError>;
    fn create_folder(&self, path: &Path) -> Result<(), OperationError>;
    fn remove_folder(&self, path: &Path) -> Result<(), OperationError>;
    fn create_file(&self, path: &Path) -> Result<(), OperationError>;
    fn link(&self, target: &Path, link: &Path) -> Result<(), OperationError>;
    fn set_mode(&self, path: &Path, mode: u32) -> Result<(), OperationError>;
}
