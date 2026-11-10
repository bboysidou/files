use gpui::{FocusHandle, Task, UniformListScrollHandle};

use crate::features::browser::presentation::state::compare_state::CompareState;

pub struct CompareSlot {
    pub state: Option<CompareState>,
    pub focus: FocusHandle,
    pub task: Option<Task<()>>,
    pub scroll: UniformListScrollHandle,
}

impl CompareSlot {
    pub fn new(focus: FocusHandle) -> Self {
        Self {
            state: None,
            focus,
            task: None,
            scroll: UniformListScrollHandle::new(),
        }
    }
}
