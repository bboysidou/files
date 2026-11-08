use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DropRequest {
    pub sources: Vec<PathBuf>,
    pub destination: PathBuf,
    pub copy: bool,
}
