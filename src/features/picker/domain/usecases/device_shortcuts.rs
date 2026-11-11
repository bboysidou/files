use crate::features::devices::domain::repositories::device_repository::DeviceRepository;
use crate::features::picker::domain::entities::picker_shortcut::{
    PickerShortcut, ShortcutGroup, ShortcutLook,
};

pub fn device_shortcuts(devices: &dyn DeviceRepository) -> Vec<PickerShortcut> {
    devices
        .list()
        .into_iter()
        .filter_map(|device| {
            device.mount.map(|path| PickerShortcut {
                group: ShortcutGroup::Devices,
                look: ShortcutLook::Named(device.label),
                path,
            })
        })
        .collect()
}
