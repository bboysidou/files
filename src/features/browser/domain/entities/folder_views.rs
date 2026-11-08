use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::enums::view::view_mode::ViewMode;

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FolderViews {
    pub enabled: bool,
    pub views: BTreeMap<String, ViewMode>,
}

impl FolderViews {
    pub fn get(&self, folder: &Path) -> Option<ViewMode> {
        self.views.get(folder.to_string_lossy().as_ref()).copied()
    }

    pub fn record(&mut self, folder: &Path, view: ViewMode) {
        if self.enabled {
            self.views
                .insert(folder.to_string_lossy().into_owned(), view);
        }
    }
}
