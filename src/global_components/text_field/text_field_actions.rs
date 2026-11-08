use gpui::{KeyBinding, actions};

actions!(
    text_field,
    [
        Backspace,
        Delete,
        DeleteWordBackward,
        MoveLeft,
        MoveRight,
        SelectLeft,
        SelectRight,
        MoveToStart,
        MoveToEnd,
        SelectAll,
        Copy,
        Cut,
        Paste,
        Submit,
        Cancel,
    ]
);

pub const TEXT_FIELD_CONTEXT: &str = "TextField";

pub fn text_field_key_bindings() -> Vec<KeyBinding> {
    let context = Some(TEXT_FIELD_CONTEXT);
    vec![
        KeyBinding::new("backspace", Backspace, context),
        KeyBinding::new("delete", Delete, context),
        KeyBinding::new("ctrl-backspace", DeleteWordBackward, context),
        KeyBinding::new("left", MoveLeft, context),
        KeyBinding::new("right", MoveRight, context),
        KeyBinding::new("shift-left", SelectLeft, context),
        KeyBinding::new("shift-right", SelectRight, context),
        KeyBinding::new("home", MoveToStart, context),
        KeyBinding::new("end", MoveToEnd, context),
        KeyBinding::new("ctrl-a", SelectAll, context),
        KeyBinding::new("ctrl-c", Copy, context),
        KeyBinding::new("ctrl-x", Cut, context),
        KeyBinding::new("ctrl-v", Paste, context),
        KeyBinding::new("enter", Submit, context),
        KeyBinding::new("down", Submit, context),
        KeyBinding::new("escape", Cancel, context),
    ]
}
