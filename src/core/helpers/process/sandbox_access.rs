use std::path::PathBuf;

#[derive(Default)]
pub struct SandboxAccess {
    pub read: Vec<PathBuf>,
    pub write: Vec<PathBuf>,
}
