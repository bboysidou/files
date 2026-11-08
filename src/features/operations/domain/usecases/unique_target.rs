use std::path::{Path, PathBuf};

use crate::core::constants::files::DUPLICATE_FIRST_NUMBER;

pub fn unique_target(directory: &Path, name: &str, taken: &dyn Fn(&Path) -> bool) -> PathBuf {
    let first = directory.join(name);
    if !taken(&first) {
        return first;
    }
    let (stem, extension) = match name.rfind('.') {
        Some(dot) if dot > 0 => name.split_at(dot),
        _ => (name, ""),
    };
    (DUPLICATE_FIRST_NUMBER..)
        .map(|number| directory.join(format!("{stem} ({number}){extension}")))
        .find(|candidate| !taken(candidate))
        .unwrap_or(first)
}
