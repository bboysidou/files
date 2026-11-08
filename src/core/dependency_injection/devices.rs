use std::sync::Arc;

use crate::features::devices::data::repositories::udisks_device_repository::UdisksDeviceRepository;
use crate::features::devices::domain::repositories::device_repository::DeviceRepository;

pub fn device_repository() -> Arc<dyn DeviceRepository> {
    Arc::new(UdisksDeviceRepository)
}
