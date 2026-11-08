use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use gpui::Task;

use crate::features::browser::domain::entities::file_entry::FileEntry;

#[derive(Default)]
pub struct TreeState {
    pub expanded: HashSet<PathBuf>,
    pub children: HashMap<PathBuf, Vec<FileEntry>>,
    pub depths: Vec<usize>,
    pub task: Option<Task<()>>,
}

impl TreeState {
    pub fn depth(&self, index: usize) -> usize {
        self.depths.get(index).copied().unwrap_or_default()
    }
}
