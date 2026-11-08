use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::core::enums::browser::sort_key::SortKey;
use crate::core::enums::view::view_mode::ViewMode;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SavedTab {
    Overview,
    Trash,
    Directory { path: PathBuf },
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show_hidden: bool,
    pub sort_key: SortKey,
    pub sort_ascending: bool,
    pub view_mode: ViewMode,
    pub sidebar_open: bool,
    pub preview_open: bool,
    pub active_tab: usize,
    pub tabs: Vec<SavedTab>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_hidden: false,
            sort_key: SortKey::Name,
            sort_ascending: true,
            view_mode: ViewMode::List,
            sidebar_open: true,
            preview_open: true,
            active_tab: 0,
            tabs: Vec::new(),
        }
    }
}
