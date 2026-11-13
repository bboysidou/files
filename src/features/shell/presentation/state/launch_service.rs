use gpui::{Global, Task};

pub struct LaunchService {
    pub _task: Task<()>,
}

impl Global for LaunchService {}
