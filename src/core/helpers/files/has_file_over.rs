use std::fs;
use std::path::Path;

pub fn has_file_over(path: &Path, limit: u64) -> bool {
    let Ok(meta) = fs::symlink_metadata(path) else {
        return false;
    };
    if meta.is_file() {
        return meta.len() > limit;
    }
    meta.is_dir()
        && fs::read_dir(path)
            .map(|items| {
                items
                    .flatten()
                    .any(|item| has_file_over(&item.path(), limit))
            })
            .unwrap_or(false)
}
