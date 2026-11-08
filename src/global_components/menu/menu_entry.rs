use gpui::{Action, SharedString};

use crate::core::enums::assets::icon::Icon;

pub struct MenuEntry {
    pub label: SharedString,
    pub icon: Icon,
    pub shortcut: SharedString,
    pub action: Box<dyn Action>,
    pub enabled: bool,
    pub danger: bool,
    pub separated: bool,
}
