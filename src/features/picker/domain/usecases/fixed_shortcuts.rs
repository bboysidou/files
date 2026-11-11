use std::path::PathBuf;

use crate::core::helpers::files::display_name::display_name;
use crate::features::network::domain::usecases::network_mount_root::network_mount_root;
use crate::features::picker::domain::entities::picker_shortcut::{
    PickerShortcut, ShortcutGroup, ShortcutLook,
};
use crate::features::places::domain::entities::place::{Place, PlaceTarget};

pub fn fixed_shortcuts(places: Vec<Place>, favorites: Vec<PathBuf>) -> Vec<PickerShortcut> {
    let places = places.into_iter().filter_map(|place| match place.target {
        PlaceTarget::Directory(path) => Some(PickerShortcut {
            group: ShortcutGroup::Places,
            look: ShortcutLook::Place(place.kind),
            path,
        }),
        PlaceTarget::Overview | PlaceTarget::Trash => None,
    });
    let favorites = favorites
        .into_iter()
        .filter(|path| network_mount_root(path).is_none())
        .map(|path| PickerShortcut {
            group: ShortcutGroup::Favorites,
            look: ShortcutLook::Named(display_name(&path)),
            path,
        });
    places.chain(favorites).collect()
}
