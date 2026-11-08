use std::sync::Arc;

use crate::features::operations::data::repositories::local_file_operations::LocalFileOperations;
use crate::features::operations::data::repositories::wayland_system_clipboard::WaylandSystemClipboard;
use crate::features::operations::domain::repositories::file_operations::FileOperations;
use crate::features::operations::domain::repositories::system_clipboard::SystemClipboard;

pub fn file_operations() -> Arc<dyn FileOperations> {
    Arc::new(LocalFileOperations)
}

pub fn system_clipboard() -> Arc<dyn SystemClipboard> {
    Arc::new(WaylandSystemClipboard)
}
