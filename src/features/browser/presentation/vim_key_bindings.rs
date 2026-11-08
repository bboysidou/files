use gpui::KeyBinding;

use crate::features::browser::presentation::actions::*;
use crate::features::browser::presentation::key_bindings::PANE_KEYS;

pub fn vim_key_bindings() -> Vec<KeyBinding> {
    let mut bindings = vim_movement();
    bindings.extend(vim_commands());
    bindings
}

fn vim_movement() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("j", MoveDown, context),
        KeyBinding::new("k", MoveUp, context),
        KeyBinding::new("g g", SelectFirst, context),
        KeyBinding::new("shift-g", SelectLast, context),
        KeyBinding::new("shift-j", PreviewPageDown, context),
        KeyBinding::new("shift-k", PreviewPageUp, context),
        KeyBinding::new("l", OpenEntry, context),
        KeyBinding::new("h", GoParent, context),
        KeyBinding::new("/", OpenFilter, context),
        KeyBinding::new("f", StartSearch, context),
        KeyBinding::new(":", EditPath, context),
        KeyBinding::new("m", OpenMenu, context),
        KeyBinding::new("?", ShowKeymap, context),
        KeyBinding::new(",", ShowSettings, context),
        KeyBinding::new("v", ToggleMark, context),
        KeyBinding::new("shift-v", InvertSelection, context),
        KeyBinding::new("s", MeasureFolders, context),
        KeyBinding::new("y", CopySelection, context),
        KeyBinding::new("x", CutSelection, context),
        KeyBinding::new("p", PasteClipboard, context),
        KeyBinding::new("d d", TrashSelection, context),
        KeyBinding::new("r", RenameEntry, context),
        KeyBinding::new("shift-n", NewFile, context),
        KeyBinding::new("o", OpenWith, context),
        KeyBinding::new("i", ShowProperties, context),
        KeyBinding::new("e", ExtractHere, context),
    ]
}

fn vim_commands() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("c", CompressZip, context),
        KeyBinding::new("shift-b", BrowseArchive, context),
        KeyBinding::new("shift-s", OpenTerminal, context),
        KeyBinding::new("z", Undo, context),
        KeyBinding::new("t", NewTab, context),
        KeyBinding::new("w", CloseTab, context),
        KeyBinding::new("]", NextTab, context),
        KeyBinding::new("[", PreviousTab, context),
        KeyBinding::new("1", ActivateTab1, context),
        KeyBinding::new("2", ActivateTab2, context),
        KeyBinding::new("3", ActivateTab3, context),
        KeyBinding::new("4", ActivateTab4, context),
        KeyBinding::new("5", ActivateTab5, context),
        KeyBinding::new("6", ActivateTab6, context),
        KeyBinding::new("7", ActivateTab7, context),
        KeyBinding::new("8", ActivateTab8, context),
        KeyBinding::new("9", ActivateTab9, context),
        KeyBinding::new("{", MoveTabLeft, context),
        KeyBinding::new("}", MoveTabRight, context),
        KeyBinding::new("shift-t 1", TagRed, context),
        KeyBinding::new("shift-t 2", TagOrange, context),
        KeyBinding::new("shift-t 3", TagYellow, context),
        KeyBinding::new("shift-t 4", TagGreen, context),
        KeyBinding::new("shift-t 5", TagBlue, context),
        KeyBinding::new("shift-t 6", TagPurple, context),
        KeyBinding::new("shift-t 7", TagGrey, context),
        KeyBinding::new("shift-t 0", ClearTags, context),
    ]
}
