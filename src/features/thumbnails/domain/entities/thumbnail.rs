use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Thumbnail {
    Ready(PathBuf),
    Failed,
}
