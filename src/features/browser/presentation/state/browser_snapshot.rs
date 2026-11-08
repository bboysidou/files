use crate::core::enums::browser::key_preset::KeyPreset;
use crate::features::browser::domain::entities::folder_views::FolderViews;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BrowserSnapshot {
    pub tabs: Vec<TabSnapshot>,
    pub active: usize,
    pub options: ViewOptions,
    pub preview_open: bool,
    pub dual_split: u16,
    pub restore_tabs: bool,
    pub key_preset: KeyPreset,
    pub folder_views: FolderViews,
}
