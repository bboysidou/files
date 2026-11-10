pub const BADGE_MAX_CHARS: usize = 4;
pub const BADGE_FALLBACK: &str = "FILE";
pub const HIDDEN_PREFIX: char = '.';
pub const SIZE_STEP: f64 = 1024.0;
pub const SIZE_UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];
pub const SIZE_ONE_DECIMAL_BELOW: f64 = 10.0;
pub const TRASH_FILES_DIR: &str = "Trash/files";
pub const OPENER_COMMAND: &str = "xdg-open";
pub const MISSING_VALUE: &str = "—";
pub const DUPLICATE_FIRST_NUMBER: usize = 2;
pub const FORBIDDEN_NAME_CHARS: [char; 2] = ['/', '\0'];
pub const RESERVED_NAMES: [&str; 2] = [".", ".."];
pub const WATCH_DEBOUNCE_MS: u64 = 250;
pub const TRASH_INFO_DIR: &str = "info";
pub const TRASH_INFO_EXTENSION: &str = "trashinfo";
pub const COPY_BUFFER_BYTES: usize = 1024 * 1024;
pub const XDG_TAGS_ATTRIBUTE: &str = "user.xdg.tags";
pub const XDG_TAGS_SEPARATOR: char = ',';
pub const OVERVIEW_WALK_LIMIT: usize = 100_000;
pub const OVERVIEW_RECENT_LIMIT: usize = 12;
pub const DF_COMMAND: &str = "df";
pub const DF_ARGUMENTS: [&str; 12] = [
    "-B1",
    "--output=source,target,size,used",
    "-x",
    "tmpfs",
    "-x",
    "devtmpfs",
    "-x",
    "efivarfs",
    "-x",
    "overlay",
    "-x",
    "squashfs",
];
pub const RECENT_FILES_NAME: &str = "recently-used.xbel";
pub const USER_DATABASE: &str = "/etc/passwd";
pub const GROUP_DATABASE: &str = "/etc/group";
pub const ACCOUNT_FIELD_SEPARATOR: char = ':';
pub const PERMISSION_BITS: u32 = 0o777;
pub const PERMISSION_TOP_BIT: u32 = 0o400;
pub const PERMISSION_COUNT: usize = 9;
pub const BAND_SCROLL_TICK_MS: u64 = 16;
pub const TYPE_AHEAD_RESET_MS: u64 = 1000;
pub const KINETIC_RELEASE_MS: u64 = 50;
pub const KINETIC_SAMPLE_WINDOW_MS: u128 = 100;
pub const KINETIC_MIN_SPAN_MS: f32 = 16.0;
pub const KINETIC_START_SPEED: f32 = 300.0;
pub const KINETIC_STOP_SPEED: f32 = 20.0;
pub const KINETIC_FRICTION: f32 = 0.95;
pub const MILLIS_PER_SECOND: f32 = 1000.0;
pub const WRITEBACK_WINDOW_BYTES: u64 = 8 * 1024 * 1024;
pub const SYS_DEV_BLOCK: &str = "/sys/dev/block";
pub const REMOVABLE_FLAG_FILE: &str = "removable";
pub const REMOVABLE_FLAG_ON: &str = "1";
pub const DEVICE_SLAVES_DIR: &str = "slaves";
pub const USB_SYSFS_SEGMENT: &str = "/usb";
pub const SLOW_FILESYSTEM_MAGICS: [u32; 4] = [0x6969, 0xFF53_4D42, 0xFE53_4D42, 0x6573_5546];
pub const TAB_MARKER_DIR: &str = "files";
pub const TAB_MARKER_PREFIX: &str = "tab-";
pub const TEAR_MIN_TABS: usize = 2;
pub const SPLIT_MIN_TABS: usize = 2;
pub const MSDOS_MAGIC: u32 = 0x4d44;
pub const EXFAT_MAGIC: u32 = 0x2011_BAB0;
pub const NTFS3_MAGIC: u32 = 0x7366_746e;
pub const NTFS_MAGIC: u32 = 0x5346_544e;
pub const FUSE_MAGIC: u32 = 0x6573_5546;
pub const NETWORK_MAGICS: [u32; 3] = [0x6969, 0xFF53_4D42, 0xFE53_4D42];
pub const MOUNTINFO_PATH: &str = "/proc/self/mountinfo";
pub const MOUNTINFO_SEPARATOR: &str = " - ";
pub const MOUNTINFO_MOUNT_POINT_FIELD: usize = 4;
pub const FUSEBLK_TYPE: &str = "fuseblk";
pub const FUSE_NETWORK_TYPES: [&str; 3] = ["fuse.rclone", "fuse.sshfs", "fuse.gvfsd-fuse"];
pub const FAT_FORBIDDEN_CHARS: [char; 9] = ['"', '*', ':', '<', '>', '?', '\\', '|', '/'];
pub const FAT_REPLACEMENT_CHAR: char = '_';
pub const FAT_TRAILING_CHARS: [char; 2] = ['.', ' '];
pub const FAT_MAX_FILE_BYTES: u64 = u32::MAX as u64;
pub const CASE_RENAME_TEMP_SUFFIX: &str = ".files-tmp";
pub const COMPARE_TEXT_MAX_BYTES: u64 = 2 * 1024 * 1024;
