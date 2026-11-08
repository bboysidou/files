use std::path::PathBuf;

pub trait NetworkMounter: Send + Sync {
    fn mount(&self, uri: &str) -> Result<PathBuf, String>;
}
