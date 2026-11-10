use std::path::PathBuf;

use crate::core::constants::files::TAB_MARKER_PREFIX;
use crate::core::helpers::files::tab_marker_path::tab_marker_path;

pub fn tab_marker_token(paths: &[PathBuf]) -> Option<u64> {
    let [path] = paths else {
        return None;
    };
    let token = path
        .file_name()?
        .to_str()?
        .strip_prefix(TAB_MARKER_PREFIX)?
        .parse::<u64>()
        .ok()?;
    (tab_marker_path(token).as_ref() == Some(path)).then_some(token)
}
