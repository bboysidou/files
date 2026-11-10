use std::fs;
use std::path::Path;

use crate::core::constants::files::{
    DEVICE_SLAVES_DIR, REMOVABLE_FLAG_FILE, REMOVABLE_FLAG_ON, USB_SYSFS_SEGMENT,
};

fn flagged(node: &Path) -> bool {
    fs::read_to_string(node.join(REMOVABLE_FLAG_FILE))
        .is_ok_and(|flag| flag.trim() == REMOVABLE_FLAG_ON)
}

pub fn removable_device(node: &Path) -> bool {
    let Ok(real) = fs::canonicalize(node) else {
        return false;
    };
    if real.to_string_lossy().contains(USB_SYSFS_SEGMENT) {
        return true;
    }
    if flagged(&real) || real.parent().is_some_and(flagged) {
        return true;
    }
    fs::read_dir(real.join(DEVICE_SLAVES_DIR))
        .map(|slaves| {
            slaves
                .flatten()
                .any(|slave| removable_device(&slave.path()))
        })
        .unwrap_or(false)
}
