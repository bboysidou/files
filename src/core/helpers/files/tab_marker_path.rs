use std::path::PathBuf;

use crate::core::constants::files::{TAB_MARKER_DIR, TAB_MARKER_PREFIX};

pub fn tab_marker_path(token: u64) -> Option<PathBuf> {
    dirs::runtime_dir().map(|dir| {
        dir.join(TAB_MARKER_DIR)
            .join(format!("{TAB_MARKER_PREFIX}{token}"))
    })
}
