use gpui::KeyBinding;

use crate::features::browser::presentation::actions::{
    ActivateTab1, ActivateTab2, ActivateTab3, ActivateTab4, ActivateTab5, ActivateTab6,
    ActivateTab7, ActivateTab8, ActivateTab9, CloseTab, GoBack, GoForward, GoParent, MoveDown,
    MoveUp, NewTab, NextTab, OpenEntry, PreviousTab, Reload, SelectFirst, SelectLast, ToggleHidden,
};

pub const BROWSER_CONTEXT: &str = "Browser";
pub const PANE_CONTEXT: &str = "Pane";

pub fn browser_key_bindings() -> Vec<KeyBinding> {
    let mut bindings = pane_bindings();
    bindings.extend(tab_bindings());
    bindings
}

fn pane_bindings() -> Vec<KeyBinding> {
    let context = Some(PANE_CONTEXT);
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
    ]
}

fn tab_bindings() -> Vec<KeyBinding> {
    let context = Some(BROWSER_CONTEXT);
    vec![
        KeyBinding::new("t", NewTab, context),
        KeyBinding::new("ctrl-t", NewTab, context),
        KeyBinding::new("w", CloseTab, context),
        KeyBinding::new("ctrl-w", CloseTab, context),
        KeyBinding::new("]", NextTab, context),
        KeyBinding::new("ctrl-tab", NextTab, context),
        KeyBinding::new("[", PreviousTab, context),
        KeyBinding::new("ctrl-shift-tab", PreviousTab, context),
        KeyBinding::new("1", ActivateTab1, context),
        KeyBinding::new("2", ActivateTab2, context),
        KeyBinding::new("3", ActivateTab3, context),
        KeyBinding::new("4", ActivateTab4, context),
        KeyBinding::new("5", ActivateTab5, context),
        KeyBinding::new("6", ActivateTab6, context),
        KeyBinding::new("7", ActivateTab7, context),
        KeyBinding::new("8", ActivateTab8, context),
        KeyBinding::new("9", ActivateTab9, context),
    ]
}
