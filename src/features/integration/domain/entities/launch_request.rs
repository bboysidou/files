use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LaunchRequest {
    Window,
    Open(PathBuf),
    Reveal(PathBuf),
    SetDefault(bool),
}
