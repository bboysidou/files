use gpui::Global;

use crate::features::places::domain::entities::sidebar_pins::SidebarPins;

#[derive(Default)]
pub struct Pins(pub SidebarPins);

impl Global for Pins {}
