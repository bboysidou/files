use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScpJob {
    pub source: String,
    pub target: String,
    pub port: Option<String>,
    pub local: PathBuf,
    pub partial: PathBuf,
}
