use gpui::KeyBinding;

use crate::core::constants::keymap::FOCUS_RAIL_KEYS;
use crate::features::shell::presentation::actions::{
    FocusRail, RailLeave, RailNext, RailOpen, RailPrevious,
};
use crate::features::shell::presentation::components::rail::RAIL_CONTEXT;

pub fn rail_key_bindings() -> Vec<KeyBinding> {
    let rail = Some(RAIL_CONTEXT);
    vec![
        KeyBinding::new(FOCUS_RAIL_KEYS, FocusRail, None),
        KeyBinding::new("j", RailNext, rail),
        KeyBinding::new("down", RailNext, rail),
        KeyBinding::new("k", RailPrevious, rail),
        KeyBinding::new("up", RailPrevious, rail),
        KeyBinding::new("enter", RailOpen, rail),
        KeyBinding::new("l", RailOpen, rail),
        KeyBinding::new("right", RailOpen, rail),
        KeyBinding::new("escape", RailLeave, rail),
        KeyBinding::new("h", RailLeave, rail),
        KeyBinding::new("left", RailLeave, rail),
    ]
}
