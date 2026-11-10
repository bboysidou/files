use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::core::constants::network::{GIO_COMMAND, GIO_MOUNT, GIO_UNMOUNT_FLAG};
use crate::features::network::data::datasources::gio_local_path::gio_local_path;
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
        gio_local_path(uri).ok_or_else(|| NetworkMountError::Failed(uri.to_owned()))
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
