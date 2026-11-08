use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::core::constants::network::{
    GIO_COMMAND, GIO_INFO, GIO_MOUNT, GIO_UNMOUNT_FLAG, LOCAL_PATH_PREFIX,
};
use crate::features::network::data::datasources::gio_mount_session::gio_mount_session;
use crate::features::network::data::datasources::gvfs_mounts::gvfs_mounts;
use crate::features::network::domain::entities::network_credentials::NetworkCredentials;
use crate::features::network::domain::entities::network_mount::NetworkMount;
use crate::features::network::domain::entities::network_mount_error::NetworkMountError;
use crate::features::network::domain::repositories::network_mounter::NetworkMounter;

pub struct GioNetworkMounter;

impl NetworkMounter for GioNetworkMounter {
    fn mount(
        &self,
        uri: &str,
        credentials: Option<&NetworkCredentials>,
    ) -> Result<PathBuf, NetworkMountError> {
        gio_mount_session(uri, credentials)?;
        let info = Command::new(GIO_COMMAND)
            .args([GIO_INFO, uri])
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .map_err(|error| NetworkMountError::Failed(error.to_string()))?;
        String::from_utf8_lossy(&info.stdout)
            .lines()
            .find_map(|line| line.strip_prefix(LOCAL_PATH_PREFIX))
            .map(|path| PathBuf::from(path.trim()))
            .ok_or_else(|| NetworkMountError::Failed(uri.to_owned()))
    }

    fn mounts(&self) -> Vec<NetworkMount> {
        gvfs_mounts()
    }

    fn unmount(&self, path: &Path) -> Result<(), String> {
        let output = Command::new(GIO_COMMAND)
            .arg(GIO_MOUNT)
            .arg(GIO_UNMOUNT_FLAG)
            .arg(path)
            .stdin(Stdio::null())
            .output()
            .map_err(|error| error.to_string())?;
        if output.status.success() {
            return Ok(());
        }
        Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
    }
}
