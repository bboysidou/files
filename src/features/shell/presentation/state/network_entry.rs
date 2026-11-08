use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkEntry {
    pub label: String,
    pub uri: Option<String>,
    pub mount: Option<PathBuf>,
}
