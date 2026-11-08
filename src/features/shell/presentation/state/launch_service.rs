use std::any::Any;

use gpui::{Global, Task};

pub struct LaunchService {
    pub _bus: Option<Box<dyn Any>>,
    pub _task: Task<()>,
}

impl Global for LaunchService {}
