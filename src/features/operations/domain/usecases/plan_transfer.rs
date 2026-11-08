use std::path::{Path, PathBuf};

use super::unique_target::unique_target;
use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::enums::operations::collision_policy::CollisionPolicy;
use crate::features::operations::domain::entities::file_operation::{FileOperation, Transfer};
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::entities::transfer_request::TransferRequest;

pub fn count_collisions(request: &TransferRequest, exists: &dyn Fn(&Path) -> bool) -> usize {
    request
        .sources
        .iter()
        .filter(|source| {
            !(request.mode == ClipboardMode::Cut
                && source.parent() == Some(request.destination.as_path()))
        })
        .filter_map(|source| source.file_name())
        .filter(|name| exists(&request.destination.join(name)))
        .count()
}

pub fn plan_transfer(
    request: &TransferRequest,
    exists: &dyn Fn(&Path) -> bool,
) -> Result<FileOperation, OperationError> {
    let destination = request.destination.as_path();
    let mut planned: Vec<PathBuf> = Vec::new();
    let mut replaced: Vec<PathBuf> = Vec::new();
    let mut transfers = Vec::new();
    for source in &request.sources {
        if destination.starts_with(source) {
            return Err(OperationError::IntoItself);
        }
        if request.mode == ClipboardMode::Cut && source.parent() == Some(destination) {
            continue;
        }
        let name = source
            .file_name()
            .ok_or(OperationError::Failed)?
            .to_string_lossy()
            .into_owned();
        let direct = destination.join(&name);
        let target = match (request.policy, exists(&direct)) {
            (CollisionPolicy::Skip, true) => continue,
            (CollisionPolicy::Replace, true) => {
                replaced.push(direct.clone());
                direct
            }
            _ => {
                let taken = |candidate: &Path| {
                    exists(candidate) || planned.iter().any(|path| path == candidate)
                };
                unique_target(destination, &name, &taken)
            }
        };
        planned.push(target.clone());
        transfers.push(Transfer {
            from: source.clone(),
            to: target,
        });
    }
    let transfer = match request.mode {
        ClipboardMode::Copy => FileOperation::Copy(transfers),
        ClipboardMode::Cut => FileOperation::Move(transfers),
    };
    Ok(if replaced.is_empty() {
        transfer
    } else {
        FileOperation::Batch(vec![FileOperation::Trash(replaced), transfer])
    })
}
