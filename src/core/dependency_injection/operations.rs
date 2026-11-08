use std::sync::Arc;

use crate::features::operations::data::repositories::freedesktop_trash_bin::FreedesktopTrashBin;
use crate::features::operations::data::repositories::local_file_operations::LocalFileOperations;
use crate::features::operations::data::repositories::wayland_system_clipboard::WaylandSystemClipboard;
use crate::features::operations::domain::repositories::file_operations::FileOperations;
use crate::features::operations::domain::repositories::system_clipboard::SystemClipboard;
use crate::features::operations::domain::repositories::trash_bin::TrashBin;

pub fn file_operations() -> Arc<dyn FileOperations> {
    Arc::new(LocalFileOperations)
}

pub fn system_clipboard() -> Arc<dyn SystemClipboard> {
    Arc::new(WaylandSystemClipboard)
}

pub fn trash_bin() -> Arc<dyn TrashBin> {
    Arc::new(FreedesktopTrashBin)
}
