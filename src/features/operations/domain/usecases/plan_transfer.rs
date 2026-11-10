use std::path::{Path, PathBuf};

use super::plan_merge::{MergePlan, plan_merge};
use super::unique_target::unique_target;
use crate::core::constants::files::FAT_MAX_FILE_BYTES;
use crate::core::enums::files::filesystem_kind::FilesystemKind;
use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::enums::operations::collision_policy::CollisionPolicy;
use crate::core::helpers::files::fat_safe_name::fat_safe_name;
use crate::features::operations::domain::entities::file_operation::{FileOperation, Transfer};
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::entities::transfer_request::TransferRequest;
use crate::features::operations::domain::repositories::file_operations::FileOperations;

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

fn fits_on(
    kind: FilesystemKind,
    request: &TransferRequest,
    files: &dyn FileOperations,
) -> Result<(), OperationError> {
    let too_large = kind == FilesystemKind::Fat
        && request
            .sources
            .iter()
            .any(|source| files.has_file_over(source, FAT_MAX_FILE_BYTES));
    if too_large {
        Err(OperationError::TooLargeForFat)
    } else {
        Ok(())
    }
}

fn target_name(source: &Path, kind: FilesystemKind) -> Result<String, OperationError> {
    let name = source
        .file_name()
        .ok_or(OperationError::Failed)?
        .to_string_lossy()
        .into_owned();
    Ok(if kind.restricts_names() {
        fat_safe_name(&name)
    } else {
        name
    })
}

pub fn plan_transfer(
    request: &TransferRequest,
    files: &dyn FileOperations,
) -> Result<FileOperation, OperationError> {
    let exists = |path: &Path| files.exists(path);
    let destination = request.destination.as_path();
    let kind = files.filesystem_kind(destination);
    fits_on(kind, request, files)?;
    let mut planned: Vec<PathBuf> = Vec::new();
    let mut replaced: Vec<PathBuf> = Vec::new();
    let mut merge = MergePlan {
        transfers: Vec::new(),
        emptied: Vec::new(),
    };
    let mut transfers = Vec::new();
    for source in &request.sources {
        if destination.starts_with(source) {
            return Err(OperationError::IntoItself);
        }
        if request.mode == ClipboardMode::Cut && source.parent() == Some(destination) {
            continue;
        }
        let name = target_name(source, kind)?;
        let direct = destination.join(&name);
        let target = match (request.policy, exists(&direct)) {
            (CollisionPolicy::Skip, true) => continue,
            (CollisionPolicy::Merge, true) if files.is_dir(&direct) && files.is_dir(source) => {
                plan_merge(source, &direct, files, &mut merge);
                continue;
            }
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
    transfers.extend(merge.transfers);
    Ok(assemble(request.mode, transfers, replaced, merge.emptied))
}

fn assemble(
    mode: ClipboardMode,
    transfers: Vec<Transfer>,
    replaced: Vec<PathBuf>,
    emptied: Vec<PathBuf>,
) -> FileOperation {
    let transfer = match mode {
        ClipboardMode::Copy => FileOperation::Copy(transfers),
        ClipboardMode::Cut => FileOperation::Move(transfers),
    };
    let mut steps = Vec::new();
    if !replaced.is_empty() {
        steps.push(FileOperation::Trash(replaced));
    }
    steps.push(transfer);
    if mode == ClipboardMode::Cut {
        steps.extend(emptied.into_iter().map(FileOperation::RemoveFolder));
    }
    if steps.len() == 1 {
        steps.remove(0)
    } else {
        FileOperation::Batch(steps)
    }
}
