use std::path::Path;

pub trait FileLauncher {
    fn launch(&self, path: &Path);
}
