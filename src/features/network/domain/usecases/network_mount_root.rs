use std::env;
use std::path::{Path, PathBuf};

use crate::core::constants::network::{GVFS_DIR, RUNTIME_DIR_ENV};
use crate::features::network::domain::usecases::network_mount_label::network_mount_label;

pub fn network_mount_root(path: &Path) -> Option<(PathBuf, String)> {
    let gvfs = PathBuf::from(env::var_os(RUNTIME_DIR_ENV)?).join(GVFS_DIR);
    let name = path.strip_prefix(&gvfs).ok()?.components().next()?;
    let name = name.as_os_str().to_string_lossy();
    let label = network_mount_label(&name)?;
    Some((gvfs.join(name.as_ref()), label))
}
