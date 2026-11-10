use std::fs;
use std::path::{Path, PathBuf};

pub fn resolved_path(path: &Path) -> PathBuf {
    if let Ok(real) = fs::canonicalize(path) {
        return real;
    }
    match (path.parent(), path.file_name()) {
        (Some(parent), Some(name)) => resolved_path(parent).join(name),
        _ => path.to_path_buf(),
    }
}
