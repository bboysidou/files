use std::sync::Arc;

use crate::core::constants::phones::{VOLUME_MONITOR_INTERFACE, VOLUME_MONITOR_PATH};
use crate::features::network::data::repositories::gvfs_mount_watcher::GvfsMountWatcher;
use crate::features::network::domain::repositories::mount_watcher::MountWatcher;
use crate::features::phones::data::repositories::gio_phone_repository::GioPhoneRepository;
use crate::features::phones::domain::repositories::phone_repository::PhoneRepository;

pub fn phone_repository() -> Arc<dyn PhoneRepository> {
    Arc::new(GioPhoneRepository)
}

pub fn volume_watcher() -> Arc<dyn MountWatcher> {
    Arc::new(GvfsMountWatcher {
        interface: VOLUME_MONITOR_INTERFACE,
        path: VOLUME_MONITOR_PATH,
    })
}
