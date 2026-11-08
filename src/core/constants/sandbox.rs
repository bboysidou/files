pub const SANDBOX_COMMAND: &str = "bwrap";
pub const SANDBOX_BASE_ARGUMENTS: [&str; 22] = [
    "--unshare-all",
    "--die-with-parent",
    "--new-session",
    "--clearenv",
    "--ro-bind",
    "/usr",
    "/usr",
    "--symlink",
    "usr/lib",
    "/lib",
    "--symlink",
    "usr/lib64",
    "/lib64",
    "--symlink",
    "usr/bin",
    "/bin",
    "--proc",
    "/proc",
    "--dev",
    "/dev",
    "--tmpfs",
    "/tmp",
];
pub const SANDBOX_OPTIONAL_READS: [&str; 3] =
    ["/etc/fonts", "/var/cache/fontconfig", "/etc/ld.so.cache"];
pub const SANDBOX_READ_FLAG: &str = "--ro-bind";
pub const SANDBOX_WRITE_FLAG: &str = "--bind";
pub const SANDBOX_SETENV_FLAG: &str = "--setenv";
pub const SANDBOX_COMMAND_SEPARATOR: &str = "--";
pub const SANDBOX_PATH_VALUE: &str = "/usr/bin";
pub const SANDBOX_PASSED_VARIABLES: [&str; 2] = ["LANG", "LC_ALL"];
pub const SANDBOX_PROBE_PROGRAM: &str = "true";
pub const SANDBOX_SYSTEM_ROOT: &str = "/usr";
