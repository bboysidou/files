use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

use gpui::Task;

use crate::features::browser::presentation::state::thumb_state::{ThumbEntry, ThumbRequest};

#[derive(Default)]
pub struct ThumbSlot {
    pub entries: HashMap<PathBuf, ThumbEntry>,
    pub queue: VecDeque<ThumbRequest>,
    pub worker: Option<Task<()>>,
}

#[derive(Default)]
pub struct WatchSlot {
    pub dir: Option<PathBuf>,
    pub task: Option<Task<()>>,
}

#[derive(Default)]
pub struct RenameSlot {
    pub path: Option<PathBuf>,
    pub focus: bool,
    pub pending: Option<PathBuf>,
}
