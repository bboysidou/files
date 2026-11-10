use std::path::Path;

use crate::core::constants::network::{GVFS_HOST_KEY, GVFS_PORT_KEY, GVFS_USER_KEY};
use crate::core::constants::ssh::REMOTE_ROOT;
use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::features::network::domain::entities::sftp_location::SftpLocation;
use crate::features::network::domain::usecases::gvfs_mount_value::gvfs_mount_value;
use crate::features::network::domain::usecases::network_mount_root::network_mount_root;

pub fn sftp_location_of(path: &Path) -> Option<SftpLocation> {
    let (mount, _) = network_mount_root(path)?;
    let name = mount.file_name()?.to_string_lossy().into_owned();
    if NetworkProtocol::from_mount_name(&name) != Some(NetworkProtocol::Sftp) {
        return None;
    }
    let rest = path
        .strip_prefix(&mount)
        .ok()?
        .to_string_lossy()
        .into_owned();
    let value = |key: &str| gvfs_mount_value(&name, key).map(str::to_owned);
    Some(SftpLocation {
        host: value(GVFS_HOST_KEY)?,
        user: value(GVFS_USER_KEY),
        port: value(GVFS_PORT_KEY),
        remote_path: format!("{REMOTE_ROOT}{rest}"),
    })
}
