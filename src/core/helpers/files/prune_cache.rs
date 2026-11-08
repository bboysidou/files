use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::core::helpers::files::tree_size::tree_size;

pub fn prune_cache(dir: &Path, max_age: Duration, max_bytes: u64) {
    let Ok(listing) = fs::read_dir(dir) else {
        return;
    };
    let now = SystemTime::now();
    let mut kept: Vec<(SystemTime, PathBuf, u64)> = Vec::new();
    for item in listing.flatten() {
        let path = item.path();
        let modified = item
            .metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(now);
        if now.duration_since(modified).unwrap_or_default() > max_age {
            remove(&path);
        } else {
            kept.push((modified, path.clone(), tree_size(&path)));
        }
    }
    kept.sort_by_key(|(modified, _, _)| *modified);
    let mut total: u64 = kept.iter().map(|(_, _, size)| size).sum();
    for (_, path, size) in kept {
        if total <= max_bytes {
            break;
        }
        remove(&path);
        total = total.saturating_sub(size);
    }
}

fn remove(path: &Path) {
    let _ = if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
}
