use std::sync::Arc;

use gpui::Task;

use crate::features::network::domain::repositories::mount_watcher::MountWatcher;
use crate::features::phones::domain::entities::phone::Phone;
use crate::features::phones::domain::repositories::phone_repository::PhoneRepository;

pub struct PhoneRail {
    pub phones: Vec<Phone>,
    pub repository: Arc<dyn PhoneRepository>,
    pub watcher: Arc<dyn MountWatcher>,
    pub task: Option<Task<()>>,
    pub mount_task: Option<Task<()>>,
    pub watch: Option<Task<()>>,
}

impl PhoneRail {
    pub fn new(repository: Arc<dyn PhoneRepository>, watcher: Arc<dyn MountWatcher>) -> Self {
        Self {
            phones: Vec::new(),
            repository,
            watcher,
            task: None,
            mount_task: None,
            watch: None,
        }
    }
}
