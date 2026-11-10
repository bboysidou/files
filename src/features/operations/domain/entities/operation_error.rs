#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OperationError {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    IntoItself,
    Failed,
    Cancelled,
    NotEmpty,
    TooLargeForFat,
}

impl OperationError {
    pub fn label_key(self) -> &'static str {
        match self {
            OperationError::NotFound => "op.error.not_found",
            OperationError::PermissionDenied => "op.error.denied",
            OperationError::AlreadyExists => "op.error.exists",
            OperationError::IntoItself => "op.error.into_itself",
            OperationError::Failed => "op.error.failed",
            OperationError::Cancelled => "op.error.cancelled",
            OperationError::NotEmpty => "op.error.not_empty",
            OperationError::TooLargeForFat => "op.error.too_large_for_fat",
        }
    }
}
