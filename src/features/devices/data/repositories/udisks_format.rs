use std::collections::HashMap;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::Path;
use std::time::Duration;

use zbus::blocking::{Connection, Proxy, connection};
use zbus::zvariant::{OwnedObjectPath, Value};

use crate::features::devices::domain::entities::format_request::{
    Filesystem, FormatError, FormatOptions, FormatRequest, FormatTarget,
};

const SERVICE: &str = "org.freedesktop.UDisks2";
const BLOCK: &str = "org.freedesktop.UDisks2.Block";
const DRIVE: &str = "org.freedesktop.UDisks2.Drive";
const FILESYSTEM: &str = "org.freedesktop.UDisks2.Filesystem";
const MANAGER: &str = "/org/freedesktop/UDisks2/Manager";

type Options = HashMap<&'static str, Value<'static>>;

fn error(error: zbus::Error) -> FormatError {
    if let zbus::Error::MethodError(name, _, _) = &error {
        let name = name.as_str();
        if name.contains("NotAuthorized") || name.contains("Cancelled") {
            return FormatError::NotAuthorized;
        }
        if name.ends_with("DeviceBusy") {
            return FormatError::Busy;
        }
        if name.ends_with("ServiceUnknown")
            || name.ends_with("NameHasNoOwner")
            || name.ends_with("UnknownMethod")
        {
            return FormatError::Unavailable;
        }
    }
    FormatError::Failed
}

fn connect() -> Result<Connection, FormatError> {
    connection::Builder::system()
        .map_err(|_| FormatError::Unavailable)?
        .method_timeout(Duration::from_secs(60 * 60))
        .build()
        .map_err(|_| FormatError::Unavailable)
}

fn proxy<'a>(
    connection: &'a Connection,
    path: &'a str,
    interface: &'a str,
) -> Result<Proxy<'a>, FormatError> {
    Proxy::new(connection, SERVICE, path, interface).map_err(error)
}

fn manager(connection: &Connection) -> Result<Proxy<'_>, FormatError> {
    proxy(connection, MANAGER, "org.freedesktop.UDisks2.Manager")
}

// Linux assigns a new disk sequence when a disk is registered again, even if /dev/sdX
// and its filesystem UUID are reused. Fail closed when the kernel does not expose it.
fn disk_sequence(block: &Path) -> Result<u64, FormatError> {
    let name = block.file_name().ok_or(FormatError::Unsafe)?;
    let sysfs = fs::canonicalize(Path::new("/sys/class/block").join(name))
        .map_err(|_| FormatError::Changed)?;
    for path in sysfs.ancestors().take(2) {
        if let Ok(sequence) = fs::read_to_string(path.join("diskseq")) {
            return sequence.trim().parse().map_err(|_| FormatError::Unsafe);
        }
    }
    Err(FormatError::Unsafe)
}

fn block_path(connection: &Connection, block: &Path) -> Result<String, FormatError> {
    let paths: Vec<OwnedObjectPath> = manager(connection)?
        .call("GetBlockDevices", &(Options::new(),))
        .map_err(error)?;
    for path in paths {
        let candidate = proxy(connection, path.as_str(), BLOCK)?;
        let mut device: Vec<u8> = candidate.get_property("Device").map_err(error)?;
        if device.last() == Some(&0) {
            device.pop();
        }
        if device == block.as_os_str().as_bytes() {
            return Ok(path.to_string());
        }
    }
    Err(FormatError::Changed)
}

fn inspect(connection: &Connection, block: &Path) -> Result<FormatTarget, FormatError> {
    let block = fs::canonicalize(block).map_err(|_| FormatError::Changed)?;
    let metadata = fs::metadata(&block).map_err(|_| FormatError::Changed)?;
    if !metadata.file_type().is_block_device() {
        return Err(FormatError::Unsafe);
    }
    let disk_sequence = disk_sequence(&block)?;
    let object_path = block_path(connection, &block)?;
    let device = proxy(connection, &object_path, BLOCK)?;
    let drive_path: OwnedObjectPath = device.get_property("Drive").map_err(error)?;
    let drive = proxy(connection, drive_path.as_str(), DRIVE)?;
    let bus: String = drive.get_property("ConnectionBus").map_err(error)?;
    let readonly: bool = device.get_property("ReadOnly").map_err(error)?;
    let system: bool = device.get_property("HintSystem").map_err(error)?;
    let usage: String = device.get_property("IdUsage").map_err(error)?;
    let backing: OwnedObjectPath = device.get_property("CryptoBackingDevice").map_err(error)?;
    let xml = device.introspect().map_err(|_| FormatError::Failed)?;
    Eligibility {
        bus: &bus,
        readonly,
        system,
        usage: &usage,
        encrypted: backing.as_str() != "/",
        partition_table: xml.contains("org.freedesktop.UDisks2.PartitionTable"),
    }
    .check()?;
    let filesystem = proxy(connection, &object_path, FILESYSTEM)?;
    let mounts: Vec<Vec<u8>> = filesystem.get_property("MountPoints").map_err(error)?;
    if mounts.iter().any(|mount| system_mount(mount)) {
        return Err(FormatError::Unsafe);
    }
    Ok(FormatTarget {
        block,
        object_path: object_path.clone(),
        disk_sequence,
        device_number: metadata.rdev(),
        size: device.get_property("Size").map_err(error)?,
        uuid: device.get_property("IdUUID").map_err(error)?,
        drive_id: drive.get_property("Id").map_err(error)?,
    })
}

struct Eligibility<'a> {
    bus: &'a str,
    readonly: bool,
    system: bool,
    usage: &'a str,
    encrypted: bool,
    partition_table: bool,
}

impl Eligibility<'_> {
    fn check(&self) -> Result<(), FormatError> {
        if self.bus != "usb"
            || self.readonly
            || self.system
            || self.usage != "filesystem"
            || self.encrypted
            || self.partition_table
        {
            Err(FormatError::Unsafe)
        } else {
            Ok(())
        }
    }
}

fn system_mount(mount: &[u8]) -> bool {
    let mount = mount.strip_suffix(&[0]).unwrap_or(mount);
    [
        b"/".as_slice(),
        b"/boot",
        b"/home",
        b"/usr",
        b"/var",
        b"/etc",
    ]
    .iter()
    .any(|root| mount == *root || (*root != b"/" && mount.starts_with(&[*root, b"/"].concat())))
}

fn available(connection: &Connection, filesystem: Filesystem) -> Result<bool, FormatError> {
    let (available, _): (bool, String) = manager(connection)?
        .call("CanFormat", &(filesystem.id(),))
        .map_err(error)?;
    Ok(available)
}

pub fn options(block: &Path) -> Result<FormatOptions, FormatError> {
    let connection = connect()?;
    let target = inspect(&connection, block)?;
    let filesystems = Filesystem::ALL
        .into_iter()
        .filter(|filesystem| available(&connection, *filesystem).unwrap_or(false))
        .collect();
    Ok(FormatOptions {
        target,
        filesystems,
    })
}

pub fn format(request: &FormatRequest) -> Result<(), FormatError> {
    use crate::features::devices::domain::usecases::format_volume::format_volume;
    format_volume(&DbusFormatSession(connect()?), request)
}

struct DbusFormatSession(Connection);

impl crate::features::devices::domain::usecases::format_volume::FormatSession
    for DbusFormatSession
{
    fn inspect(&self, target: &FormatTarget) -> Result<FormatTarget, FormatError> {
        inspect(&self.0, &target.block)
    }

    fn available(&self, filesystem: Filesystem) -> Result<bool, FormatError> {
        available(&self.0, filesystem)
    }

    fn mounted(&self, target: &FormatTarget) -> Result<bool, FormatError> {
        let volume = proxy(&self.0, &target.object_path, FILESYSTEM)?;
        let mounts: Vec<Vec<u8>> = volume.get_property("MountPoints").map_err(error)?;
        Ok(!mounts.is_empty())
    }

    fn unmount(&self, target: &FormatTarget) -> Result<(), FormatError> {
        // Never force an unmount or tear down children or configuration entries.
        proxy(&self.0, &target.object_path, FILESYSTEM)?
            .call::<_, _, ()>("Unmount", &(Options::new(),))
            .map_err(error)
    }

    fn create(&self, request: &FormatRequest) -> Result<(), FormatError> {
        proxy(&self.0, &request.target.object_path, BLOCK)?
            .call::<_, _, ()>(
                "Format",
                &(
                    request.filesystem.id(),
                    format_arguments(request.filesystem, &request.label),
                ),
            )
            .map_err(error)
    }
}

fn format_arguments(filesystem: Filesystem, label: &str) -> Options {
    let mut options = HashMap::from([("update-partition-type", Value::Bool(true))]);
    options.insert("label", Value::from(label.to_owned()));
    if filesystem == Filesystem::Ext4 {
        options.insert("take-ownership", Value::Bool(true));
    }
    if filesystem == Filesystem::Fat32 {
        // UDisks exposes vfat, without a dedicated FAT variant option.
        options.insert("mkfs-args", Value::from(vec!["-F", "32"]));
    }
    options
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_usb_readonly_system_encrypted_and_partitioned_devices() {
        let eligible = || Eligibility {
            bus: "usb",
            readonly: false,
            system: false,
            usage: "filesystem",
            encrypted: false,
            partition_table: false,
        };
        assert_eq!(eligible().check(), Ok(()));
        let excluded = [
            Eligibility {
                bus: "ata",
                ..eligible()
            },
            Eligibility {
                readonly: true,
                ..eligible()
            },
            Eligibility {
                system: true,
                ..eligible()
            },
            Eligibility {
                encrypted: true,
                ..eligible()
            },
            Eligibility {
                partition_table: true,
                ..eligible()
            },
            Eligibility {
                usage: "crypto",
                ..eligible()
            },
            Eligibility {
                usage: "",
                ..eligible()
            },
        ];
        for device in excluded {
            assert_eq!(device.check(), Err(FormatError::Unsafe));
        }
    }

    #[test]
    fn format_arguments_force_fat32_and_give_ext4_ownership_without_tearing_down() {
        let fat = format_arguments(Filesystem::Fat32, "MES_DOCS");
        assert_eq!(fat.get("label"), Some(&Value::from("MES_DOCS")));
        assert_eq!(fat.get("mkfs-args"), Some(&Value::from(vec!["-F", "32"])));
        assert_eq!(
            format_arguments(Filesystem::Ext4, "").get("take-ownership"),
            Some(&Value::Bool(true))
        );
        for filesystem in Filesystem::ALL {
            let options = format_arguments(filesystem, "");
            assert_eq!(
                options.get("update-partition-type"),
                Some(&Value::Bool(true))
            );
            assert!(!options.contains_key("tear-down"));
            assert!(!options.contains_key("erase"));
            assert!(!options.contains_key("no-block"));
        }
    }

    #[test]
    fn decodes_the_single_struct_returned_by_can_format() {
        let reply = zbus::Message::method_call("/test", "CanFormat")
            .unwrap()
            .build(&((true, String::new()),))
            .unwrap();
        assert_eq!(reply.body().signature().to_string(), "(bs)");
        let (available, _): (bool, String) = reply.body().deserialize().unwrap();
        assert!(available);
    }

    #[test]
    fn protects_system_mounts_without_rejecting_similar_names() {
        for path in [b"/\0".as_slice(), b"/boot/efi\0", b"/home", b"/var/lib"] {
            assert!(system_mount(path));
        }
        for path in [
            b"/media/usb\0".as_slice(),
            b"/run/media/user/usb\0",
            b"/homework",
            b"/mnt/usb",
        ] {
            assert!(!system_mount(path));
        }
    }
}
