use crate::features::browser::presentation::state::setting_change::SettingChange;

pub enum SettingControl {
    Toggle {
        on: bool,
        change: SettingChange,
    },
    Choice {
        labels: Vec<&'static str>,
        selected: usize,
        pick: fn(usize) -> Option<SettingChange>,
    },
}

pub struct SettingItem {
    pub id: &'static str,
    pub label: &'static str,
    pub control: SettingControl,
}
