use std::sync::Arc;

use crate::core::constants::network::{GVFS_TRACKER_INTERFACE, GVFS_TRACKER_PATH};
use crate::features::network::data::repositories::gio_network_mounter::GioNetworkMounter;
use crate::features::network::data::repositories::gvfs_mount_watcher::GvfsMountWatcher;
use crate::features::network::domain::repositories::mount_watcher::MountWatcher;
use crate::features::network::domain::repositories::network_mounter::NetworkMounter;

pub fn network_mounter() -> Arc<dyn NetworkMounter> {
    Arc::new(GioNetworkMounter)
}

pub fn mount_watcher() -> Arc<dyn MountWatcher> {
    Arc::new(GvfsMountWatcher {
        interface: GVFS_TRACKER_INTERFACE,
        path: GVFS_TRACKER_PATH,
    })
}
