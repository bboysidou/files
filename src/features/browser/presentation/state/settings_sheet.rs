use gpui::FocusHandle;

use crate::features::browser::domain::entities::folder_views::FolderViews;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;

pub struct SettingsSheet {
    pub open: bool,
    pub focus: FocusHandle,
    pub restore_tabs: bool,
    pub folder_views: FolderViews,
}

impl SettingsSheet {
    pub fn closed(focus: FocusHandle, restore: &BrowserSnapshot) -> Self {
        Self {
            open: false,
            focus,
            restore_tabs: restore.restore_tabs,
            folder_views: restore.folder_views.clone(),
        }
    }
}
