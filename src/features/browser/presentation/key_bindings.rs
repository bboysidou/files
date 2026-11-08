use gpui::KeyBinding;

use crate::features::browser::presentation::actions::{
    GoBack, GoForward, GoParent, MoveDown, MoveUp, OpenEntry, Reload, SelectFirst, SelectLast,
    ToggleHidden,
};

pub const BROWSER_CONTEXT: &str = "Browser";

pub fn browser_key_bindings() -> Vec<KeyBinding> {
    let context = Some(BROWSER_CONTEXT);
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
