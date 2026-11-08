use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IndexedPath {
    pub path: PathBuf,
    pub name_lower: String,
}
