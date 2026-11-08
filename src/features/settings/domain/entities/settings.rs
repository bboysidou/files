use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::core::constants::settings::DUAL_SPLIT_DEFAULT_PERMILLE;
use crate::core::enums::browser::key_preset::KeyPreset;
use crate::core::enums::browser::sort_key::SortKey;
use crate::core::enums::view::view_mode::ViewMode;
use crate::features::browser::domain::entities::folder_views::FolderViews;
use crate::features::browser::domain::entities::list_columns::ListColumns;
use crate::features::network::domain::entities::saved_server::SavedServer;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SavedTab {
    Overview,
    Trash,
    Directory { path: PathBuf },
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SavedStop {
    #[serde(flatten)]
    pub target: SavedTab,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus: Option<PathBuf>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SavedTabState {
    #[serde(flatten)]
    pub location: SavedTab,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub back: Vec<SavedStop>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forward: Vec<SavedStop>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show_hidden: bool,
    pub sort_key: SortKey,
    pub sort_ascending: bool,
    pub hidden_last: bool,
    pub restore_tabs: bool,
    pub key_preset: KeyPreset,
    pub view_mode: ViewMode,
    pub columns: ListColumns,
    pub folder_views: FolderViews,
    pub servers: Vec<SavedServer>,
    pub sidebar_open: bool,
    pub preview_open: bool,
    pub dual_split: u16,
    pub active_tab: usize,
    pub tabs: Vec<SavedTabState>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_hidden: false,
            sort_key: SortKey::Name,
            sort_ascending: true,
            hidden_last: false,
            restore_tabs: true,
            key_preset: KeyPreset::Vim,
            view_mode: ViewMode::List,
            columns: ListColumns::default(),
            folder_views: FolderViews::default(),
            servers: Vec::new(),
            sidebar_open: true,
            preview_open: true,
            dual_split: DUAL_SPLIT_DEFAULT_PERMILLE,
            active_tab: 0,
            tabs: Vec::new(),
        }
    }
}
