use std::fs;
use std::path::Path;

use md5::{Digest, Md5};

use crate::core::helpers::files::file_uri::file_uri;

pub fn cache_name(path: &Path) -> Option<String> {
    let modified = fs::metadata(path).and_then(|meta| meta.modified()).ok()?;
    let stamp = format!("{}:{:?}", file_uri(path), modified);
    Some(format!("{:x}", Md5::digest(stamp.as_bytes())))
}
