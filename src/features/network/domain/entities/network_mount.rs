use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkMount {
    pub label: String,
    pub path: PathBuf,
}
