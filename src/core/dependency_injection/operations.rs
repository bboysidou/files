use std::sync::Arc;

use crate::core::dependency_injection::admin::pkexec_session;
use crate::features::admin::data::repositories::admin_file_operations::AdminFileOperations;
use crate::features::operations::data::repositories::freedesktop_trash_bin::FreedesktopTrashBin;
use crate::features::operations::data::repositories::local_file_operations::LocalFileOperations;
use crate::features::operations::data::repositories::local_run_inspector::LocalRunInspector;
use crate::features::operations::data::repositories::wayland_system_clipboard::WaylandSystemClipboard;
use crate::features::operations::domain::repositories::file_operations::FileOperations;
use crate::features::operations::domain::repositories::run_inspector::RunInspector;
use crate::features::operations::domain::repositories::system_clipboard::SystemClipboard;
use crate::features::operations::domain::repositories::trash_bin::TrashBin;

pub fn file_operations() -> Arc<dyn FileOperations> {
    Arc::new(AdminFileOperations::new(
        Arc::new(LocalFileOperations),
        pkexec_session(),
    ))
}

pub fn system_clipboard() -> Arc<dyn SystemClipboard> {
    Arc::new(WaylandSystemClipboard)
}

pub fn trash_bin() -> Arc<dyn TrashBin> {
    Arc::new(FreedesktopTrashBin)
}

pub fn run_inspector() -> Arc<dyn RunInspector> {
    Arc::new(LocalRunInspector)
}
