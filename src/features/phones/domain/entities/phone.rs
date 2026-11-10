use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Phone {
    pub name: String,
    pub uri: String,
    pub mount: Option<PathBuf>,
}
