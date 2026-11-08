use std::path::{Path, PathBuf};

use super::unique_target::unique_target;
use crate::features::operations::domain::entities::file_operation::Transfer;
use crate::features::operations::domain::repositories::file_operations::FileOperations;

pub struct MergePlan {
    pub transfers: Vec<Transfer>,
    pub emptied: Vec<PathBuf>,
}

pub fn plan_merge(source: &Path, target: &Path, files: &dyn FileOperations, plan: &mut MergePlan) {
    for child in files.children(source) {
        let Some(name) = child
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
        else {
            continue;
        };
        let direct = target.join(&name);
        if files.exists(&direct) && files.is_dir(&direct) && files.is_dir(&child) {
            plan_merge(&child, &direct, files, plan);
            continue;
        }
        let taken = |candidate: &Path| {
            files.exists(candidate)
                || plan
                    .transfers
                    .iter()
                    .any(|transfer| transfer.to == candidate)
        };
        let to = unique_target(target, &name, &taken);
        plan.transfers.push(Transfer { from: child, to });
    }
    plan.emptied.push(source.to_path_buf());
}
