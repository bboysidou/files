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
