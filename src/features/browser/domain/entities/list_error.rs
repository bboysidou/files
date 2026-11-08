#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ListError {
    NotFound,
    PermissionDenied,
    Failed,
}
