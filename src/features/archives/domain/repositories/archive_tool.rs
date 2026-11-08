use std::path::{Path, PathBuf};

pub trait ArchiveTool: Send + Sync {
    fn extract(&self, archive: &Path, destination: &Path) -> Result<(), String>;
    fn compress(&self, sources: &[PathBuf], archive: &Path) -> Result<(), String>;
}
