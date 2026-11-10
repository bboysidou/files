use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::core::constants::devices::{
    MOUNT_USERS_MAX, PROC_COMM, PROC_CWD, PROC_DIR, PROC_FD_DIR,
};

fn holds(process: &Path, mount: &Path) -> bool {
    let inside = |link: &Path| fs::read_link(link).is_ok_and(|target| target.starts_with(mount));
    inside(&process.join(PROC_CWD))
        || fs::read_dir(process.join(PROC_FD_DIR))
            .map(|fds| fds.flatten().any(|fd| inside(&fd.path())))
            .unwrap_or(false)
}

pub fn mount_users(mount: &Path) -> Vec<String> {
    let Ok(processes) = fs::read_dir(PROC_DIR) else {
        return Vec::new();
    };
    let names: BTreeSet<String> = processes
        .flatten()
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .bytes()
                .all(|b| b.is_ascii_digit())
        })
        .map(|entry| entry.path())
        .filter(|process| holds(process, mount))
        .filter_map(|process| fs::read_to_string(process.join(PROC_COMM)).ok())
        .map(|name| name.trim().to_string())
        .collect();
    names.into_iter().take(MOUNT_USERS_MAX).collect()
}
