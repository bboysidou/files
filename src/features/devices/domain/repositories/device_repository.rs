use std::path::PathBuf;

use crate::features::devices::domain::entities::device::Device;
use crate::features::devices::domain::entities::device_error::DeviceError;

pub trait DeviceRepository: Send + Sync {
    fn list(&self) -> Vec<Device>;
    fn mount(&self, device: &Device) -> Result<PathBuf, DeviceError>;
    fn eject(&self, device: &Device) -> Result<(), DeviceError>;
}
