use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filesystem {
    Fat32,
    Exfat,
    Ntfs,
    Ext4,
}

impl Filesystem {
    pub const ALL: [Self; 4] = [Self::Fat32, Self::Exfat, Self::Ntfs, Self::Ext4];

    pub fn id(self) -> &'static str {
        match self {
            Self::Fat32 => "vfat",
            Self::Exfat => "exfat",
            Self::Ntfs => "ntfs",
            Self::Ext4 => "ext4",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Fat32 => "FAT32",
            Self::Exfat => "exFAT",
            Self::Ntfs => "NTFS",
            Self::Ext4 => "ext4",
        }
    }
}

/// Captured when the dialog opens and checked again immediately before formatting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormatTarget {
    pub block: PathBuf,
    pub object_path: String,
    pub disk_sequence: u64,
    pub device_number: u64,
    pub size: u64,
    pub uuid: String,
    pub drive_id: String,
}

#[derive(Clone, Debug)]
pub struct FormatOptions {
    pub target: FormatTarget,
    pub filesystems: Vec<Filesystem>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatError {
    InvalidLabel,
    Unavailable,
    Unsafe,
    Changed,
    Busy,
    NotAuthorized,
    Failed,
}

impl FormatError {
    pub fn label_key(self) -> &'static str {
        match self {
            Self::InvalidLabel => "device.format.error.label",
            Self::Unavailable => "device.format.error.unavailable",
            Self::Unsafe => "device.format.error.unsafe",
            Self::Changed => "device.format.error.changed",
            Self::Busy => "device.error.busy",
            Self::NotAuthorized => "device.format.error.authorization",
            Self::Failed => "device.format.error.failed",
        }
    }
}

#[derive(Clone, Debug)]
pub struct FormatRequest {
    pub target: FormatTarget,
    pub filesystem: Filesystem,
    pub label: String,
}
