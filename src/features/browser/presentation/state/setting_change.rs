use crate::core::enums::browser::key_preset::KeyPreset;
use crate::core::enums::browser::sort_key::SortKey;
use crate::core::enums::view::view_mode::ViewMode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SettingChange {
    ShowHidden(bool),
    HiddenLast(bool),
    SortBy(SortKey),
    Ascending(bool),
    View(ViewMode),
    PreviewColumn,
    RestoreTabs(bool),
    Column(SortKey, bool),
    FolderViews(bool),
    Keys(KeyPreset),
}
