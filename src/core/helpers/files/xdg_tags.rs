use std::path::Path;

use crate::core::constants::files::{XDG_TAGS_ATTRIBUTE, XDG_TAGS_SEPARATOR};

pub fn xdg_tags(path: &Path) -> Vec<String> {
    let Ok(Some(raw)) = xattr::get(path, XDG_TAGS_ATTRIBUTE) else {
        return Vec::new();
    };
    String::from_utf8_lossy(&raw)
        .split(XDG_TAGS_SEPARATOR)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .collect()
}
