use gpui::FocusHandle;

pub struct SettingsSheet {
    pub open: bool,
    pub focus: FocusHandle,
    pub restore_tabs: bool,
}

impl SettingsSheet {
    pub fn closed(focus: FocusHandle, restore_tabs: bool) -> Self {
        Self {
            open: false,
            focus,
            restore_tabs,
        }
    }
}
