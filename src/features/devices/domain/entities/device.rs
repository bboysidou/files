use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Device {
    pub block: PathBuf,
    pub drive: Option<PathBuf>,
    pub label: String,
    pub mount: Option<PathBuf>,
}
