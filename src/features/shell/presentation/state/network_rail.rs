use std::sync::Arc;

use gpui::Task;

use crate::features::network::domain::entities::network_mount::NetworkMount;
use crate::features::network::domain::repositories::network_mounter::NetworkMounter;
use crate::features::places::domain::entities::place::PlaceTarget;

pub struct NetworkRail {
    pub mounts: Vec<NetworkMount>,
    pub mounter: Arc<dyn NetworkMounter>,
    pub seen: Option<PlaceTarget>,
    pub task: Option<Task<()>>,
}
