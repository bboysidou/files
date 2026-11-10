use std::fs;
use std::path::Path;

use crate::core::constants::files::{
    MOUNTINFO_MOUNT_POINT_FIELD, MOUNTINFO_PATH, MOUNTINFO_SEPARATOR,
};

pub fn fuse_type_of(path: &Path) -> Option<String> {
    let text = fs::read_to_string(MOUNTINFO_PATH).ok()?;
    text.lines()
        .filter_map(|line| {
            let (left, right) = line.split_once(MOUNTINFO_SEPARATOR)?;
            let mount = left.split(' ').nth(MOUNTINFO_MOUNT_POINT_FIELD)?;
            let fstype = right.split(' ').next()?;
            path.starts_with(mount)
                .then(|| (mount.len(), fstype.to_string()))
        })
        .max_by_key(|(length, _)| *length)
        .map(|(_, fstype)| fstype)
}
