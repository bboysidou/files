use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::core::constants::devices::{
    BUSY_MARKER, DEVICE_PREFIX, DISK_TYPE, LSBLK_ARGS, LSBLK_COMMAND, PARTITION_TYPE,
    SWAP_FILESYSTEM, UDISKS_BLOCK_FLAG, UDISKS_COMMAND, UDISKS_MOUNT, UDISKS_MOUNTED_AT,
    UDISKS_NO_PROMPT, UDISKS_POWER_OFF, UDISKS_UNMOUNT,
};
use crate::core::helpers::files::sync_folder::sync_folder;
use crate::core::helpers::process::mount_users::mount_users;
use crate::features::devices::data::models::lsblk_node::{LsblkNode, LsblkReport};
use crate::features::devices::domain::entities::device::Device;
use crate::features::devices::domain::entities::device_error::DeviceError;
use crate::features::devices::domain::repositories::device_repository::DeviceRepository;

use crate::features::devices::data::repositories::udisks_format;
use crate::features::devices::domain::entities::format_request::{
    FormatError, FormatOptions, FormatRequest, FormatTarget,
};

pub struct UdisksDeviceRepository;

impl UdisksDeviceRepository {
    fn udisks(verb: &str, block: &Path) -> Result<Output, DeviceError> {
        let output = Command::new(UDISKS_COMMAND)
            .args([verb, UDISKS_NO_PROMPT, UDISKS_BLOCK_FLAG])
            .arg(block)
            .output()
            .map_err(|_| DeviceError::Failed)?;
        if output.status.success() {
            return Ok(output);
        }
        let message = String::from_utf8_lossy(&output.stderr).to_lowercase();
        Err(if message.contains(BUSY_MARKER) {
            DeviceError::Busy
        } else {
            DeviceError::Failed
        })
    }

    fn collect(node: &LsblkNode, removable_parent: bool, devices: &mut Vec<Device>) {
        let removable = removable_parent || node.hotplug || node.rm;
        let usable = node
            .fstype
            .as_deref()
            .is_some_and(|filesystem| filesystem != SWAP_FILESYSTEM);
        let leaf =
            node.kind == PARTITION_TYPE || (node.kind == DISK_TYPE && node.children.is_empty());
        if removable && usable && leaf {
            devices.push(Device {
                block: PathBuf::from(&node.path),
                drive: node
                    .pkname
                    .as_ref()
                    .map(|parent| PathBuf::from(format!("{DEVICE_PREFIX}{parent}"))),
                label: node
                    .label
                    .clone()
                    .or_else(|| node.size.clone())
                    .unwrap_or_else(|| node.path.clone()),
                mount: node.mountpoint.as_ref().map(PathBuf::from),
            });
        }
        for child in &node.children {
            Self::collect(child, removable, devices);
        }
    }
}

impl DeviceRepository for UdisksDeviceRepository {
    fn format_options(&self, device: &Device) -> Result<FormatOptions, FormatError> {
        udisks_format::options(&device.block)
    }

    fn format(&self, request: &FormatRequest) -> Result<(), FormatError> {
        udisks_format::format(request)
    }

    fn format_progress(&self, target: &FormatTarget) -> Option<f32> {
        super::udisks_format_progress::progress(&target.object_path)
    }

    fn list(&self) -> Vec<Device> {
        let Ok(output) = Command::new(LSBLK_COMMAND).args(LSBLK_ARGS).output() else {
            return Vec::new();
        };
        let Ok(report) = serde_json::from_slice::<LsblkReport>(&output.stdout) else {
            return Vec::new();
        };
        let mut devices = Vec::new();
        for node in &report.blockdevices {
            Self::collect(node, false, &mut devices);
        }
        devices
    }

    fn mount(&self, device: &Device) -> Result<PathBuf, DeviceError> {
        if let Some(mount) = &device.mount {
            return Ok(mount.clone());
        }
        let output = Self::udisks(UDISKS_MOUNT, &device.block)?;
        let text = String::from_utf8_lossy(&output.stdout);
        text.split_once(UDISKS_MOUNTED_AT)
            .map(|(_, path)| PathBuf::from(path.trim().trim_end_matches('.')))
            .ok_or(DeviceError::Failed)
    }

    fn eject(&self, device: &Device) -> Result<(), DeviceError> {
        if device.mount.is_some() {
            Self::udisks(UDISKS_UNMOUNT, &device.block)?;
        }
        if let Some(drive) = &device.drive {
            let _ = Self::udisks(UDISKS_POWER_OFF, drive);
        }
        Ok(())
    }

    fn flush(&self, mount: &Path) -> Result<(), DeviceError> {
        sync_folder(mount).map_err(|_| DeviceError::Failed)
    }

    fn users(&self, mount: &Path) -> Vec<String> {
        mount_users(mount)
    }
}
