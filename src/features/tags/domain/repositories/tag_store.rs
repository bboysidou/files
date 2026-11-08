use std::path::Path;

pub trait TagStore: Send + Sync {
    fn names(&self, path: &Path) -> Vec<String>;
    fn set_names(&self, path: &Path, names: &[String]) -> Result<(), String>;
}
