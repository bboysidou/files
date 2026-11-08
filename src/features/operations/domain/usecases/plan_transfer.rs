use std::path::{Path, PathBuf};

use super::unique_target::unique_target;
use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::features::operations::domain::entities::file_operation::{FileOperation, Transfer};
use crate::features::operations::domain::entities::operation_error::OperationError;

pub fn plan_transfer(
    sources: &[PathBuf],
    destination: &Path,
    mode: ClipboardMode,
    exists: &dyn Fn(&Path) -> bool,
) -> Result<FileOperation, OperationError> {
    let mut planned: Vec<PathBuf> = Vec::new();
    let mut transfers = Vec::new();
    for source in sources {
        if destination.starts_with(source) {
            return Err(OperationError::IntoItself);
        }
        if mode == ClipboardMode::Cut && source.parent() == Some(destination) {
            continue;
        }
        let name = source
            .file_name()
            .ok_or(OperationError::Failed)?
            .to_string_lossy()
            .into_owned();
        let taken =
            |candidate: &Path| exists(candidate) || planned.iter().any(|path| path == candidate);
        let target = unique_target(destination, &name, &taken);
        planned.push(target.clone());
        transfers.push(Transfer {
            from: source.clone(),
            to: target,
        });
    }
    Ok(match mode {
        ClipboardMode::Copy => FileOperation::Copy(transfers),
        ClipboardMode::Cut => FileOperation::Move(transfers),
    })
}
