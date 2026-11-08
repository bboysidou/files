use gpui::Global;

use crate::core::enums::browser::key_preset::KeyPreset;

pub struct ActiveKeyPreset(pub KeyPreset);

impl Global for ActiveKeyPreset {}
