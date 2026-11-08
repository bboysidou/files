use std::path::{Path, PathBuf};

use crate::features::network::domain::entities::network_credentials::NetworkCredentials;
use crate::features::network::domain::entities::network_mount::NetworkMount;
use crate::features::network::domain::entities::network_mount_error::NetworkMountError;

pub trait NetworkMounter: Send + Sync {
    fn mount(
        &self,
        uri: &str,
        credentials: Option<&NetworkCredentials>,
    ) -> Result<PathBuf, NetworkMountError>;
    fn mounts(&self) -> Vec<NetworkMount>;
    fn unmount(&self, path: &Path) -> Result<(), String>;
}
