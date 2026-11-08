use std::env;
use std::fs;
use std::path::PathBuf;

use crate::core::constants::network::{GVFS_DIR, RUNTIME_DIR_ENV};
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
            let label = network_mount_label(&entry.file_name().to_string_lossy())?;
            Some(NetworkMount {
                label,
                path: entry.path(),
            })
        })
        .collect();
    mounts.sort_by(|left, right| left.label.cmp(&right.label));
    mounts
}
