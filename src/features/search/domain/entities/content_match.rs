use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContentMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
}
