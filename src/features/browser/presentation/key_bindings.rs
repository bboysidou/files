use gpui::KeyBinding;

use crate::core::enums::browser::key_preset::KeyPreset;
use crate::features::browser::presentation::actions::*;
use crate::features::browser::presentation::classic_key_bindings::classic_key_bindings;
use crate::features::browser::presentation::vim_key_bindings::vim_key_bindings;

pub const BROWSER_CONTEXT: &str = "Browser";
pub const PANE_CONTEXT: &str = "Pane";
pub const PANE_GRID_CONTEXT: &str = "Pane view=grid";
const PANE_GRID_KEYS: &str = "Pane && view == grid && !TextField";
pub const QUICK_LOOK_CONTEXT: &str = "QuickLook";
pub const MENU_CONTEXT: &str = "Menu";
pub const KEYMAP_CONTEXT: &str = "Keymap";
pub const CONFIRM_CONTEXT: &str = "Confirm";
pub const SETTINGS_CONTEXT: &str = "Settings";
pub const BULK_CONTEXT: &str = "BulkRename";
pub const LOGIN_CONTEXT: &str = "NetworkLogin";
pub const SERVER_EDIT_CONTEXT: &str = "ServerEdit";
pub const PROPERTIES_CONTEXT: &str = "Properties";
pub(super) const PANE_KEYS: &str = "Pane && !TextField";

pub fn browser_key_bindings(preset: KeyPreset) -> Vec<KeyBinding> {
    let mut bindings = match preset {
        KeyPreset::Vim => vim_key_bindings(),
        KeyPreset::Classic => classic_key_bindings(),
    };
    bindings.extend(navigation_bindings());
    bindings.extend(grid_bindings());
    bindings.extend(selection_bindings());
    bindings.extend(operation_bindings());
    bindings.extend(tab_bindings());
    bindings.extend(quick_look_bindings());
    bindings.extend(overlay_bindings());
    bindings
}

fn navigation_bindings() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("down", MoveDown, context),
        KeyBinding::new("up", MoveUp, context),
        KeyBinding::new("home", SelectFirst, context),
        KeyBinding::new("end", SelectLast, context),
        KeyBinding::new("enter", OpenEntry, context),
        KeyBinding::new("right", OpenEntry, context),
        KeyBinding::new("left", GoParent, context),
        KeyBinding::new("backspace", GoParent, context),
        KeyBinding::new("alt-up", GoParent, context),
        KeyBinding::new("alt-left", GoBack, context),
        KeyBinding::new("alt-right", GoForward, context),
        KeyBinding::new("ctrl-h", ToggleHidden, context),
        KeyBinding::new("f5", Reload, context),
        KeyBinding::new("space", QuickLook, context),
    ]
}

fn selection_bindings() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("escape", Dismiss, context),
        KeyBinding::new("ctrl-a", SelectAllEntries, context),
        KeyBinding::new("shift-down", ExtendDown, context),
        KeyBinding::new("shift-up", ExtendUp, context),
        KeyBinding::new("shift-right", ExpandFolder, context),
        KeyBinding::new("shift-left", CollapseFolder, context),
    ]
}

fn operation_bindings() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("ctrl-c", CopySelection, context),
        KeyBinding::new("ctrl-x", CutSelection, context),
        KeyBinding::new("ctrl-v", PasteClipboard, context),
        KeyBinding::new("delete", TrashSelection, context),
        KeyBinding::new("f2", RenameEntry, context),
        KeyBinding::new("ctrl-shift-n", NewFolder, context),
        KeyBinding::new("ctrl-d", Duplicate, context),
        KeyBinding::new("ctrl-shift-v", PasteLink, context),
        KeyBinding::new("ctrl-z", Undo, context),
        KeyBinding::new("ctrl-shift-z", Redo, context),
        KeyBinding::new("ctrl-y", Redo, context),
    ]
}

fn tab_bindings() -> Vec<KeyBinding> {
    let pane = Some(PANE_KEYS);
    let browser = Some(BROWSER_CONTEXT);
    vec![
        KeyBinding::new("ctrl-t", NewTab, browser),
        KeyBinding::new("ctrl-w", CloseTab, browser),
        KeyBinding::new("ctrl-tab", NextTab, browser),
        KeyBinding::new("ctrl-shift-tab", PreviousTab, browser),
        KeyBinding::new("ctrl-shift-pageup", MoveTabLeft, browser),
        KeyBinding::new("ctrl-shift-pagedown", MoveTabRight, browser),
        KeyBinding::new("f3", TogglePreview, browser),
        KeyBinding::new("ctrl-1", ShowList, browser),
        KeyBinding::new("ctrl-2", ShowColumns, browser),
        KeyBinding::new("ctrl-3", ShowGrid, browser),
        KeyBinding::new("ctrl-4", ShowDual, browser),
        KeyBinding::new("tab", SwitchPane, pane),
        KeyBinding::new("f6", CopyToOtherPane, pane),
        KeyBinding::new("shift-f6", MoveToOtherPane, pane),
        KeyBinding::new("ctrl-l", EditPath, browser),
        KeyBinding::new("ctrl-k", ConnectServer, browser),
        KeyBinding::new("ctrl-f", StartSearch, browser),
        KeyBinding::new("ctrl-shift-c", CopyPath, browser),
        KeyBinding::new("ctrl-,", ShowSettings, browser),
    ]
}

fn quick_look_bindings() -> Vec<KeyBinding> {
    let context = Some(QUICK_LOOK_CONTEXT);
    vec![
        KeyBinding::new("space", CloseQuickLook, context),
        KeyBinding::new("escape", CloseQuickLook, context),
        KeyBinding::new("right", QuickLookNext, context),
        KeyBinding::new("l", QuickLookNext, context),
        KeyBinding::new("left", QuickLookPrevious, context),
        KeyBinding::new("h", QuickLookPrevious, context),
        KeyBinding::new("down", QuickLookScrollDown, context),
        KeyBinding::new("j", QuickLookScrollDown, context),
        KeyBinding::new("up", QuickLookScrollUp, context),
        KeyBinding::new("k", QuickLookScrollUp, context),
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
        KeyBinding::new("escape", CloseSettings, Some(SETTINGS_CONTEXT)),
        KeyBinding::new("tab", BulkNextField, Some(BULK_CONTEXT)),
        KeyBinding::new("tab", LoginNextField, Some(LOGIN_CONTEXT)),
        KeyBinding::new("tab", ServerEditNextField, Some(SERVER_EDIT_CONTEXT)),
        KeyBinding::new("escape", CloseProperties, Some(PROPERTIES_CONTEXT)),
        KeyBinding::new("enter", ConfirmAccept, Some(CONFIRM_CONTEXT)),
        KeyBinding::new("escape", ConfirmCancel, Some(CONFIRM_CONTEXT)),
        KeyBinding::new("m", CollisionMerge, Some(CONFIRM_CONTEXT)),
        KeyBinding::new("s", CollisionSkip, Some(CONFIRM_CONTEXT)),
        KeyBinding::new("r", CollisionReplace, Some(CONFIRM_CONTEXT)),
    ]
}
