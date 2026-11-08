use gpui::{Pixels, Point};

use crate::global_components::menu::menu_entry::MenuEntry;

pub struct MenuState {
    pub position: Point<Pixels>,
    pub entries: Vec<MenuEntry>,
    pub hot: usize,
}
