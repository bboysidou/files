use std::path::{Path, PathBuf};

use crate::features::operations::domain::entities::operation_error::OperationError;

pub trait FileOperations: Send + Sync {
    fn exists(&self, path: &Path) -> bool;
    fn copy(&self, from: &Path, to: &Path) -> Result<(), OperationError>;
    fn move_path(&self, from: &Path, to: &Path) -> Result<(), OperationError>;
    fn trash(&self, paths: &[PathBuf]) -> Result<(), OperationError>;
    fn restore(&self, paths: &[PathBuf]) -> Result<(), OperationError>;
    fn create_folder(&self, path: &Path) -> Result<(), OperationError>;
    fn remove_folder(&self, path: &Path) -> Result<(), OperationError>;
}
