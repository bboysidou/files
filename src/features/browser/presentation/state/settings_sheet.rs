use gpui::FocusHandle;

use crate::core::enums::browser::key_preset::KeyPreset;
use crate::features::browser::domain::entities::folder_views::FolderViews;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;

pub struct SettingsSheet {
    pub open: bool,
    pub focus: FocusHandle,
    pub restore_tabs: bool,
    pub key_preset: KeyPreset,
    pub folder_views: FolderViews,
}

impl SettingsSheet {
    pub fn closed(focus: FocusHandle, restore: &BrowserSnapshot) -> Self {
        Self {
            open: false,
            focus,
            restore_tabs: restore.restore_tabs,
            key_preset: restore.key_preset,
            folder_views: restore.folder_views.clone(),
        }
    }
}
