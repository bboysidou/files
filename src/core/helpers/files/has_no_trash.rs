use std::env;
use std::path::{Path, PathBuf};

use crate::core::constants::network::{GVFS_DIR, RUNTIME_DIR_ENV};

pub fn has_no_trash(path: &Path) -> bool {
    env::var_os(RUNTIME_DIR_ENV)
        .map(|runtime| PathBuf::from(runtime).join(GVFS_DIR))
        .is_some_and(|gvfs| path.starts_with(gvfs))
}
