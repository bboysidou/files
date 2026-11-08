use std::sync::Arc;

use gpui::{Subscription, Task};

use crate::features::network::domain::entities::network_mount::NetworkMount;
use crate::features::network::domain::entities::saved_server::SavedServer;
use crate::features::network::domain::repositories::mount_watcher::MountWatcher;
use crate::features::network::domain::repositories::network_mounter::NetworkMounter;
use crate::features::places::domain::entities::place::PlaceTarget;

pub struct NetworkRail {
    pub mounts: Vec<NetworkMount>,
    pub mounter: Arc<dyn NetworkMounter>,
    pub seen: Option<PlaceTarget>,
    pub task: Option<Task<()>>,
    pub unmount_task: Option<Task<()>>,
    pub saved: Vec<SavedServer>,
    pub events: Option<Subscription>,
    pub watcher: Arc<dyn MountWatcher>,
    pub watch: Option<Task<()>>,
}

impl NetworkRail {
    pub fn new(mounter: Arc<dyn NetworkMounter>, watcher: Arc<dyn MountWatcher>) -> Self {
        Self {
            mounts: Vec::new(),
            mounter,
            seen: None,
            task: None,
            unmount_task: None,
            saved: Vec::new(),
            events: None,
            watcher,
            watch: None,
        }
    }
}
