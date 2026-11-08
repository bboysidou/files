#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NetworkMountError {
    NeedsCredentials,
    Rejected,
    TimedOut,
    Failed(String),
}
