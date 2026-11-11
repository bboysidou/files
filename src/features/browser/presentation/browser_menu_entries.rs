use gpui::{Action, SharedString};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::features::browser::presentation::actions::ToggleFavorite;
use crate::features::browser::presentation::actions::{
    BrowseArchive, CompressTarGz, CompressZip, Duplicate, ExtractHere, NewFile, NewFolder,
    OpenTerminal, TrashSelection,
};
use crate::global_components::menu::menu_entry::MenuEntry;

pub(super) fn entry(
    label: &str,
    icon: Icon,
    shortcut: &'static str,
    action: Box<dyn Action>,
) -> MenuEntry {
    MenuEntry {
        label: t!(label).to_string().into(),
        icon,
        shortcut: SharedString::new_static(shortcut),
        action,
        enabled: true,
        danger: false,
        separated: false,
    }
}

pub(super) fn folder_entries(
    has_directory: bool,
    archive: bool,
    has_entry: bool,
) -> [MenuEntry; 9] {
    let new_folder = entry(
        "menu.new_folder",
        Icon::NewFolder,
        "Ctrl+Shift+N",
        Box::new(NewFolder),
    );
    let new_file = entry("menu.new_file", Icon::Document, "N", Box::new(NewFile));
    let duplicate = entry("menu.duplicate", Icon::Copy, "Ctrl+D", Box::new(Duplicate));
    let terminal = entry("menu.terminal", Icon::Terminal, "S", Box::new(OpenTerminal));
    let zip = entry(
        "menu.compress_zip",
        Icon::Download,
        "c",
        Box::new(CompressZip),
    );
    let tar = entry(
        "menu.compress_tar",
        Icon::Download,
        "",
        Box::new(CompressTarGz),
    );
    let browse = entry(
        "menu.browse_archive",
        Icon::Folder,
        "B",
        Box::new(BrowseArchive),
    );
    let extract = entry("menu.extract", Icon::Open, "e", Box::new(ExtractHere));
    let mut trash = entry("menu.trash", Icon::Trash, "dd", Box::new(TrashSelection));
    (trash.danger, trash.separated) = (true, true);
    [
        gated(new_folder, has_directory),
        gated(new_file, has_directory),
        gated(duplicate, has_entry),
        gated(terminal, has_directory),
        gated(extract, archive),
        gated(browse, archive),
        gated(zip, has_entry),
        gated(tar, has_entry),
        gated(trash, has_entry),
    ]
}

pub(super) fn gated(mut item: MenuEntry, enabled: bool) -> MenuEntry {
    item.enabled = enabled;
    item
}

pub(super) fn favorite_entry(target: bool, favorite: bool) -> MenuEntry {
    let label = if favorite {
        "menu.remove_favorite"
    } else {
        "menu.add_favorite"
    };
    gated(
        entry(label, Icon::Star, "Ctrl+Shift+D", Box::new(ToggleFavorite)),
        target,
    )
}
