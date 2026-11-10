#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DiffKind {
    Same,
    Removed,
    Added,
    Changed,
}
