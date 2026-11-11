use std::sync::Arc;
use std::time::Instant;

use gpui::{RenderImage, Task};

#[derive(Default)]
pub struct PlayerState {
    pub frame: Option<Arc<RenderImage>>,
    pub retired: Option<Arc<RenderImage>>,
    pub offset_ms: u64,
    pub started: Option<Instant>,
    pub task: Option<Task<()>>,
}

impl PlayerState {
    pub fn playing(&self) -> bool {
        self.task.is_some()
    }

    pub fn position_ms(&self) -> u64 {
        let elapsed = self
            .started
            .map_or(0, |started| started.elapsed().as_millis());
        self.offset_ms + u64::try_from(elapsed).unwrap_or(u64::MAX)
    }
}
