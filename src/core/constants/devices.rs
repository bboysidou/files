pub const LSBLK_COMMAND: &str = "lsblk";
pub const LSBLK_ARGS: [&str; 3] = [
    "-J",
    "-o",
    "NAME,PATH,PKNAME,LABEL,MOUNTPOINT,SIZE,FSTYPE,HOTPLUG,RM,TYPE",
];
pub const UDISKS_COMMAND: &str = "udisksctl";
pub const UDISKS_NO_PROMPT: &str = "--no-user-interaction";
pub const UDISKS_BLOCK_FLAG: &str = "-b";
pub const UDISKS_MOUNT: &str = "mount";
pub const UDISKS_UNMOUNT: &str = "unmount";
pub const UDISKS_POWER_OFF: &str = "power-off";
pub const UDISKS_MOUNTED_AT: &str = " at ";
pub const BUSY_MARKER: &str = "busy";
pub const PARTITION_TYPE: &str = "part";
pub const DISK_TYPE: &str = "disk";
pub const SWAP_FILESYSTEM: &str = "swap";
pub const DEVICE_PREFIX: &str = "/dev/";
pub const DEVICE_EVENTS_DIR: &str = "/dev/disk/by-uuid";
pub const DEVICE_REFRESH_DEBOUNCE_MS: u64 = 500;
