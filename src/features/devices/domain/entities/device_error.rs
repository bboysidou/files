#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceError {
    Busy,
    Failed,
}

impl DeviceError {
    pub fn label_key(self) -> &'static str {
        match self {
            DeviceError::Busy => "device.error.busy",
            DeviceError::Failed => "device.error.failed",
        }
    }
}
