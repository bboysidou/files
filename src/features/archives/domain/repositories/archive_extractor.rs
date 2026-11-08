use std::path::Path;

pub trait ArchiveExtractor: Send + Sync {
    fn extract(&self, archive: &Path, destination: &Path) -> Result<(), String>;
}
