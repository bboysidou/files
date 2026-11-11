use std::path::PathBuf;

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::places::place_kind::PlaceKind;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShortcutGroup {
    Places,
    Favorites,
    Devices,
}

impl ShortcutGroup {
    pub const ALL: [ShortcutGroup; 3] = [
        ShortcutGroup::Places,
        ShortcutGroup::Favorites,
        ShortcutGroup::Devices,
    ];

    pub fn label_key(self) -> &'static str {
        match self {
            ShortcutGroup::Places => "rail.section.places",
            ShortcutGroup::Favorites => "rail.section.favorites",
            ShortcutGroup::Devices => "rail.section.devices",
        }
    }

    pub fn icon(self) -> Icon {
        match self {
            ShortcutGroup::Places | ShortcutGroup::Favorites => Icon::Folder,
            ShortcutGroup::Devices => Icon::Usb,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ShortcutLook {
    Place(PlaceKind),
    Named(String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PickerShortcut {
    pub group: ShortcutGroup,
    pub look: ShortcutLook,
    pub path: PathBuf,
}
