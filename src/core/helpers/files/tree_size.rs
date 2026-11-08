use std::fs;
use std::path::Path;

pub fn tree_size(path: &Path) -> u64 {
    let Ok(meta) = fs::symlink_metadata(path) else {
        return 0;
    };
    if !meta.is_dir() {
        return meta.len();
    }
    fs::read_dir(path)
        .map(|listing| listing.flatten().map(|item| tree_size(&item.path())).sum())
        .unwrap_or_default()
}
