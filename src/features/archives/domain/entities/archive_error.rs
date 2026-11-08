#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ArchiveError {
    Cancelled,
    Failed(String),
}
