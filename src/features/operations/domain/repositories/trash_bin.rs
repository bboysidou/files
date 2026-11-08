use std::path::PathBuf;

use crate::features::operations::domain::entities::operation_error::OperationError;

pub trait TrashBin: Send + Sync {
    fn originals(&self, entries: &[PathBuf]) -> Vec<PathBuf>;
    fn purge(&self, entries: &[PathBuf]) -> Result<(), OperationError>;
    fn empty(&self) -> Result<(), OperationError>;
}
