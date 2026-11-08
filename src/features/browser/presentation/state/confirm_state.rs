use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConfirmState {
    Purge(Vec<PathBuf>),
    EmptyTrash,
}
