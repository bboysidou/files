use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlaybackRequest {
    pub path: PathBuf,
    pub start_ms: u64,
    pub poster: PathBuf,
    pub audio: bool,
}
