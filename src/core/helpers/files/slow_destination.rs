use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use rustix::fs::{major, minor, statfs};

use crate::core::constants::files::{SLOW_FILESYSTEM_MAGICS, SYS_DEV_BLOCK};
use crate::core::helpers::files::removable_device::removable_device;

fn existing_ancestor(path: &Path) -> Option<&Path> {
    path.ancestors().find(|ancestor| ancestor.exists())
}

pub fn slow_destination(path: &Path) -> bool {
    let Some(folder) = existing_ancestor(path) else {
        return false;
    };
    let network =
        statfs(folder).is_ok_and(|info| SLOW_FILESYSTEM_MAGICS.contains(&(info.f_type as u32)));
    if network {
        return true;
    }
    let Ok(meta) = fs::metadata(folder) else {
        return false;
    };
    let device = meta.dev();
    let node = PathBuf::from(SYS_DEV_BLOCK).join(format!("{}:{}", major(device), minor(device)));
    removable_device(&node)
}
