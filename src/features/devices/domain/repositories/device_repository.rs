use std::path::{Path, PathBuf};

use crate::features::devices::domain::entities::device::Device;
use crate::features::devices::domain::entities::device_error::DeviceError;
use crate::features::devices::domain::entities::format_request::{
    FormatError, FormatOptions, FormatRequest, FormatTarget,
};

pub trait DeviceRepository: Send + Sync {
    fn format_options(&self, device: &Device) -> Result<FormatOptions, FormatError>;
    fn format(&self, request: &FormatRequest) -> Result<(), FormatError>;
    fn format_progress(&self, target: &FormatTarget) -> Option<f32>;
    fn list(&self) -> Vec<Device>;
    fn mount(&self, device: &Device) -> Result<PathBuf, DeviceError>;
    fn eject(&self, device: &Device) -> Result<(), DeviceError>;
    fn flush(&self, mount: &Path) -> Result<(), DeviceError>;
    fn users(&self, mount: &Path) -> Vec<String>;
}
