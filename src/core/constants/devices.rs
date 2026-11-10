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
pub const EJECT_POLL_MS: u64 = 500;
pub const EJECT_NOTE_MIN_BYTES: u64 = 16 * 1024 * 1024;
pub const PROC_MEMINFO: &str = "/proc/meminfo";
pub const MEMINFO_DIRTY: &str = "Dirty:";
pub const MEMINFO_WRITEBACK: &str = "Writeback:";
pub const MEMINFO_UNIT_BYTES: u64 = 1024;
pub const PROC_DIR: &str = "/proc";
pub const PROC_CWD: &str = "cwd";
pub const PROC_FD_DIR: &str = "fd";
pub const PROC_COMM: &str = "comm";
pub const MOUNT_USERS_MAX: usize = 8;
pub const MOUNT_USERS_SEPARATOR: &str = ", ";
