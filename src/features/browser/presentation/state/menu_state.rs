use std::path::PathBuf;

use gpui::{Pixels, Point};

use crate::features::applications::domain::entities::desktop_app::DesktopApp;
use crate::global_components::menu::menu_entry::MenuEntry;

pub struct AppChoice {
    pub apps: Vec<DesktopApp>,
    pub paths: Vec<PathBuf>,
}

pub struct MenuState {
    pub position: Point<Pixels>,
    pub entries: Vec<MenuEntry>,
    pub hot: usize,
    pub apps: Option<AppChoice>,
}
