use std::ffi::OsString;
use std::path::{Path, PathBuf};

use trash::TrashItem;
use trash::os_limited;

use crate::core::constants::files::{TRASH_INFO_DIR, TRASH_INFO_EXTENSION};
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::repositories::trash_bin::TrashBin;

pub struct FreedesktopTrashBin;

impl FreedesktopTrashBin {
    fn info_path(entry: &Path) -> Option<OsString> {
        let trash_root = entry.parent()?.parent()?;
        let mut name = entry.file_name()?.to_os_string();
        name.push(".");
        name.push(TRASH_INFO_EXTENSION);
        Some(trash_root.join(TRASH_INFO_DIR).join(name).into_os_string())
    }

    fn matching(entries: &[PathBuf]) -> Vec<TrashItem> {
        let wanted: Vec<OsString> = entries
            .iter()
            .filter_map(|entry| Self::info_path(entry))
            .collect();
        os_limited::list()
            .unwrap_or_default()
            .into_iter()
            .filter(|item| wanted.contains(&item.id))
            .collect()
    }
}

impl TrashBin for FreedesktopTrashBin {
    fn originals(&self, entries: &[PathBuf]) -> Vec<PathBuf> {
        Self::matching(entries)
            .iter()
            .map(TrashItem::original_path)
            .collect()
    }

    fn purge(&self, entries: &[PathBuf]) -> Result<(), OperationError> {
        os_limited::purge_all(Self::matching(entries)).map_err(|_| OperationError::Failed)
    }

    fn empty(&self) -> Result<(), OperationError> {
        let items = os_limited::list().map_err(|_| OperationError::Failed)?;
        os_limited::purge_all(items).map_err(|_| OperationError::Failed)
    }
}
