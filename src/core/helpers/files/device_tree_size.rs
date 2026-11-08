use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub fn device_tree_size(path: &Path, device: u64) -> u64 {
    let Ok(meta) = fs::symlink_metadata(path) else {
        return 0;
    };
    if meta.dev() != device {
        return 0;
    }
    if !meta.is_dir() {
        return meta.len();
    }
    fs::read_dir(path)
        .map(|listing| {
            listing
                .flatten()
                .map(|item| device_tree_size(&item.path(), device))
                .sum()
        })
        .unwrap_or_default()
}
