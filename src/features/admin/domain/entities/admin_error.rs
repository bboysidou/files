#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AdminError {
    Cancelled,
    Failed,
}

impl AdminError {
    pub fn label_key(self) -> &'static str {
        match self {
            AdminError::Cancelled => "admin.error.cancelled",
            AdminError::Failed => "admin.error.failed",
        }
    }
}
