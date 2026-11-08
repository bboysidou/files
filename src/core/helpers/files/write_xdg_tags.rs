use std::io;
use std::path::Path;

use crate::core::constants::files::{XDG_TAGS_ATTRIBUTE, XDG_TAGS_SEPARATOR};
use crate::core::helpers::files::xdg_tags::xdg_tags;

pub fn write_xdg_tags(path: &Path, names: &[String]) -> io::Result<()> {
    if !names.is_empty() {
        let joined = names.join(&XDG_TAGS_SEPARATOR.to_string());
        return xattr::set(path, XDG_TAGS_ATTRIBUTE, joined.as_bytes());
    }
    if xdg_tags(path).is_empty() {
        return Ok(());
    }
    xattr::remove(path, XDG_TAGS_ATTRIBUTE)
}
