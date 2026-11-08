use gpui::KeyBinding;

use crate::features::browser::presentation::actions::*;
use crate::features::browser::presentation::key_bindings::PANE_KEYS;

pub fn classic_key_bindings() -> Vec<KeyBinding> {
    let mut bindings = classic_commands();
    bindings.extend(classic_tags());
    bindings
}

fn classic_tags() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("ctrl-alt-1", TagRed, context),
        KeyBinding::new("ctrl-alt-2", TagOrange, context),
        KeyBinding::new("ctrl-alt-3", TagYellow, context),
        KeyBinding::new("ctrl-alt-4", TagGreen, context),
        KeyBinding::new("ctrl-alt-5", TagBlue, context),
        KeyBinding::new("ctrl-alt-6", TagPurple, context),
        KeyBinding::new("ctrl-alt-7", TagGrey, context),
        KeyBinding::new("ctrl-alt-0", ClearTags, context),
    ]
}

fn classic_commands() -> Vec<KeyBinding> {
    let context = Some(PANE_KEYS);
    vec![
        KeyBinding::new("f1", ShowKeymap, context),
        KeyBinding::new("shift-f10", OpenMenu, context),
        KeyBinding::new("menu", OpenMenu, context),
        KeyBinding::new("alt-enter", ShowProperties, context),
        KeyBinding::new("ctrl-space", ToggleMark, context),
        KeyBinding::new("ctrl-i", InvertSelection, context),
        KeyBinding::new("alt-s", MeasureFolders, context),
        KeyBinding::new("ctrl-shift-f", OpenFilter, context),
        KeyBinding::new("alt-pagedown", PreviewPageDown, context),
        KeyBinding::new("alt-pageup", PreviewPageUp, context),
        KeyBinding::new("alt-1", ActivateTab1, context),
        KeyBinding::new("alt-2", ActivateTab2, context),
        KeyBinding::new("alt-3", ActivateTab3, context),
        KeyBinding::new("alt-4", ActivateTab4, context),
        KeyBinding::new("alt-5", ActivateTab5, context),
        KeyBinding::new("alt-6", ActivateTab6, context),
        KeyBinding::new("alt-7", ActivateTab7, context),
        KeyBinding::new("alt-8", ActivateTab8, context),
        KeyBinding::new("alt-9", ActivateTab9, context),
    ]
}
