use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub fn same_device(first: &Path, second: &Path) -> bool {
    match (fs::symlink_metadata(first), fs::metadata(second)) {
        (Ok(first), Ok(second)) => first.dev() == second.dev(),
        _ => false,
    }
}
