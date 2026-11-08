use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MatchLine {
    pub number: usize,
    pub text: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContentMatch {
    pub path: PathBuf,
    pub lines: Vec<MatchLine>,
    pub matches: usize,
}
