use std::time::Instant;

use gpui::Task;

#[derive(Default)]
pub struct KineticState {
    pub samples: Vec<(Instant, f32)>,
    pub task: Option<Task<()>>,
}
