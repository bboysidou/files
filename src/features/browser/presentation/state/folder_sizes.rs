use std::collections::HashMap;
use std::path::PathBuf;

use gpui::Task;

#[derive(Default)]
pub struct FolderSizes {
    pub known: HashMap<PathBuf, u64>,
    pub task: Option<Task<()>>,
}
