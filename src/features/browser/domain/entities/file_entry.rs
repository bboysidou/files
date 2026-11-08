use std::path::PathBuf;
use std::time::SystemTime;

use crate::core::enums::files::file_kind::FileKind;
use crate::core::enums::tags::tag_color::TagColor;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub extension: Option<String>,
    pub kind: FileKind,
    pub is_dir: bool,
    pub hidden: bool,
    pub size: Option<u64>,
    pub child_count: Option<usize>,
    pub modified: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub tags: Vec<TagColor>,
}
