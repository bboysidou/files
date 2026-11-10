use std::env;
use std::fs;
use std::path::PathBuf;

use crate::core::constants::network::{GVFS_DIR, GVFS_SCHEME_SEPARATOR, RUNTIME_DIR_ENV};
use crate::core::constants::phones::PHONE_SCHEMES;
use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::features::network::domain::entities::network_mount::NetworkMount;
use crate::features::network::domain::usecases::network_mount_label::network_mount_label;

pub fn gvfs_mounts() -> Vec<NetworkMount> {
    let Some(runtime) = env::var_os(RUNTIME_DIR_ENV) else {
        return Vec::new();
    };
    let Ok(entries) = fs::read_dir(PathBuf::from(runtime).join(GVFS_DIR)) else {
        return Vec::new();
    };
    let mut mounts: Vec<NetworkMount> = entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            let scheme = name.split(GVFS_SCHEME_SEPARATOR).next()?;
            if PHONE_SCHEMES.contains(&scheme) {
                return None;
            }
            let label = network_mount_label(&name)?;
            Some(NetworkMount {
                label,
                path: entry.path(),
                protocol: NetworkProtocol::from_mount_name(&name),
            })
        })
        .collect();
    mounts.sort_by(|left, right| left.label.cmp(&right.label));
    mounts
}
