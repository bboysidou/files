use gpui::{App, KeyBinding};

use crate::core::constants::keymap::{DETACH_TAB_KEYS, NEW_WINDOW_KEYS, TOGGLE_RAIL_KEYS};
use crate::core::enums::browser::key_preset::KeyPreset;
use crate::features::browser::presentation::key_bindings::browser_key_bindings;
use crate::features::browser::presentation::state::active_key_preset::ActiveKeyPreset;
use crate::features::shell::presentation::actions::{DetachTab, NewWindow, ToggleRail};
use crate::features::shell::presentation::rail_key_bindings::rail_key_bindings;
use crate::global_components::text_field::text_field_actions::text_field_key_bindings;

pub fn bind_app_keys(preset: KeyPreset, cx: &mut App) {
    cx.clear_key_bindings();
    cx.set_global(ActiveKeyPreset(preset));
    cx.bind_keys([
        KeyBinding::new(TOGGLE_RAIL_KEYS, ToggleRail, None),
        KeyBinding::new(NEW_WINDOW_KEYS, NewWindow, None),
        KeyBinding::new(DETACH_TAB_KEYS, DetachTab, None),
    ]);
    cx.bind_keys(rail_key_bindings());
    cx.bind_keys(browser_key_bindings(preset));
    cx.bind_keys(text_field_key_bindings());
}
