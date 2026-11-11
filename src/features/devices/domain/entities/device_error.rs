#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceError {
    Busy,
    NotAuthorized,
    Cancelled,
    Failed,
}

impl DeviceError {
    pub fn label_key(self) -> &'static str {
        match self {
            DeviceError::Busy => "device.error.busy",
            DeviceError::NotAuthorized => "device.error.not_authorized",
            DeviceError::Cancelled => "device.error.cancelled",
            DeviceError::Failed => "device.error.failed",
        }
    }
}
