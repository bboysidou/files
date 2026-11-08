use gpui::KeyBinding;

use crate::features::browser::presentation::actions::*;

pub const BROWSER_CONTEXT: &str = "Browser";
pub const PANE_CONTEXT: &str = "Pane";
pub const PANE_GRID_CONTEXT: &str = "Pane view=grid";
const PANE_GRID_KEYS: &str = "Pane && view == grid && !TextField";
pub const QUICK_LOOK_CONTEXT: &str = "QuickLook";
pub const MENU_CONTEXT: &str = "Menu";
pub const KEYMAP_CONTEXT: &str = "Keymap";
pub const CONFIRM_CONTEXT: &str = "Confirm";
const PANE_KEYS: &str = "Pane && !TextField";

pub fn browser_key_bindings() -> Vec<KeyBinding> {
    let mut bindings = navigation_bindings();
    bindings.extend(grid_bindings());
    bindings.extend(selection_bindings());
    bindings.extend(operation_bindings());
    bindings.extend(tab_bindings());
    bindings.extend(quick_look_bindings());
    bindings.extend(overlay_bindings());
    bindings.extend(tag_bindings());
    bindings
}

fn navigation_bindings() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("j", MoveDown, context),
        KeyBinding::new("down", MoveDown, context),
        KeyBinding::new("k", MoveUp, context),
        KeyBinding::new("up", MoveUp, context),
        KeyBinding::new("g g", SelectFirst, context),
        KeyBinding::new("home", SelectFirst, context),
        KeyBinding::new("shift-g", SelectLast, context),
        KeyBinding::new("end", SelectLast, context),
        KeyBinding::new("enter", OpenEntry, context),
        KeyBinding::new("l", OpenEntry, context),
        KeyBinding::new("right", OpenEntry, context),
        KeyBinding::new("h", GoParent, context),
        KeyBinding::new("left", GoParent, context),
        KeyBinding::new("backspace", GoParent, context),
        KeyBinding::new("alt-up", GoParent, context),
        KeyBinding::new("alt-left", GoBack, context),
        KeyBinding::new("alt-right", GoForward, context),
        KeyBinding::new("ctrl-h", ToggleHidden, context),
        KeyBinding::new("f5", Reload, context),
        KeyBinding::new("/", OpenFilter, context),
        KeyBinding::new("space", QuickLook, context),
        KeyBinding::new("f", StartSearch, context),
        KeyBinding::new(":", EditPath, context),
        KeyBinding::new("m", OpenMenu, context),
        KeyBinding::new("?", ShowKeymap, context),
    ]
}

fn selection_bindings() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("escape", Dismiss, context),
        KeyBinding::new("v", ToggleMark, context),
        KeyBinding::new("ctrl-a", SelectAllEntries, context),
        KeyBinding::new("shift-down", ExtendDown, context),
        KeyBinding::new("shift-up", ExtendUp, context),
    ]
}

fn operation_bindings() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("y", CopySelection, context),
        KeyBinding::new("ctrl-c", CopySelection, context),
        KeyBinding::new("x", CutSelection, context),
        KeyBinding::new("ctrl-x", CutSelection, context),
        KeyBinding::new("p", PasteClipboard, context),
        KeyBinding::new("ctrl-v", PasteClipboard, context),
        KeyBinding::new("d d", TrashSelection, context),
        KeyBinding::new("delete", TrashSelection, context),
        KeyBinding::new("r", RenameEntry, context),
        KeyBinding::new("f2", RenameEntry, context),
        KeyBinding::new("ctrl-shift-n", NewFolder, context),
        KeyBinding::new("z", Undo, context),
        KeyBinding::new("ctrl-z", Undo, context),
        KeyBinding::new("ctrl-shift-z", Redo, context),
        KeyBinding::new("ctrl-y", Redo, context),
    ]
}

fn tab_bindings() -> Vec<KeyBinding> {
    let pane = Some(PANE_KEYS);
    let browser = Some(BROWSER_CONTEXT);
    vec![
        KeyBinding::new("t", NewTab, pane),
        KeyBinding::new("w", CloseTab, pane),
        KeyBinding::new("]", NextTab, pane),
        KeyBinding::new("[", PreviousTab, pane),
        KeyBinding::new("1", ActivateTab1, pane),
        KeyBinding::new("2", ActivateTab2, pane),
        KeyBinding::new("3", ActivateTab3, pane),
        KeyBinding::new("4", ActivateTab4, pane),
        KeyBinding::new("5", ActivateTab5, pane),
        KeyBinding::new("6", ActivateTab6, pane),
        KeyBinding::new("7", ActivateTab7, pane),
        KeyBinding::new("8", ActivateTab8, pane),
        KeyBinding::new("9", ActivateTab9, pane),
        KeyBinding::new("ctrl-t", NewTab, browser),
        KeyBinding::new("ctrl-w", CloseTab, browser),
        KeyBinding::new("ctrl-tab", NextTab, browser),
        KeyBinding::new("ctrl-shift-tab", PreviousTab, browser),
        KeyBinding::new("f3", TogglePreview, browser),
        KeyBinding::new("ctrl-1", ShowList, browser),
        KeyBinding::new("ctrl-2", ShowColumns, browser),
        KeyBinding::new("ctrl-3", ShowGrid, browser),
        KeyBinding::new("ctrl-l", EditPath, browser),
        KeyBinding::new("ctrl-f", StartSearch, browser),
        KeyBinding::new("ctrl-shift-c", CopyPath, browser),
    ]
}

fn quick_look_bindings() -> Vec<KeyBinding> {
    let context = Some(QUICK_LOOK_CONTEXT);
    vec![
        KeyBinding::new("space", CloseQuickLook, context),
        KeyBinding::new("escape", CloseQuickLook, context),
        KeyBinding::new("right", QuickLookNext, context),
        KeyBinding::new("down", QuickLookNext, context),
        KeyBinding::new("j", QuickLookNext, context),
        KeyBinding::new("left", QuickLookPrevious, context),
        KeyBinding::new("up", QuickLookPrevious, context),
        KeyBinding::new("k", QuickLookPrevious, context),
        KeyBinding::new("+", ZoomIn, context),
        KeyBinding::new("=", ZoomIn, context),
        KeyBinding::new("-", ZoomOut, context),
    ]
}

fn grid_bindings() -> Vec<KeyBinding> {
    let context = Some(PANE_GRID_KEYS);
    vec![
        KeyBinding::new("left", MoveLeft, context),
        KeyBinding::new("right", MoveRight, context),
    ]
}

fn overlay_bindings() -> Vec<KeyBinding> {
    let menu = Some(MENU_CONTEXT);
    let keymap = Some(KEYMAP_CONTEXT);
    vec![
        KeyBinding::new("down", MenuNext, menu),
        KeyBinding::new("j", MenuNext, menu),
        KeyBinding::new("up", MenuPrevious, menu),
        KeyBinding::new("k", MenuPrevious, menu),
        KeyBinding::new("enter", MenuConfirm, menu),
        KeyBinding::new("escape", CloseMenu, menu),
        KeyBinding::new("m", CloseMenu, menu),
        KeyBinding::new("escape", CloseKeymap, keymap),
        KeyBinding::new("enter", ConfirmAccept, Some(CONFIRM_CONTEXT)),
        KeyBinding::new("escape", ConfirmCancel, Some(CONFIRM_CONTEXT)),
    ]
}

fn tag_bindings() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
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
