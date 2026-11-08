use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ThumbState {
    Pending,
    Ready(PathBuf),
    Failed,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ThumbEntry {
    pub modified: SystemTime,
    pub state: ThumbState,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ThumbRequest {
    pub path: PathBuf,
    pub modified: SystemTime,
}
