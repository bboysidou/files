pub const APPLICATIONS_DIR: &str = "applications";
pub const DESKTOP_EXTENSION: &str = "desktop";
pub const DESKTOP_ENTRY_GROUP: &str = "[Desktop Entry]";
pub const DESKTOP_ID_SEPARATOR: &str = "-";
pub const DATA_DIRS_VARIABLE: &str = "XDG_DATA_DIRS";
pub const DEFAULT_DATA_DIRS: &str = "/usr/local/share:/usr/share";
pub const DATA_DIRS_SEPARATOR: char = ':';
pub const LIST_SEPARATOR: char = ';';
pub const COMMENT_PREFIX: char = '#';
pub const GROUP_PREFIX: char = '[';
pub const KEY_SEPARATOR: char = '=';
pub const ESCAPE_MARK: char = '\\';
pub const TYPE_KEY: &str = "Type";
pub const APPLICATION_TYPE: &str = "Application";
pub const NAME_KEY: &str = "Name";
pub const EXEC_KEY: &str = "Exec";
pub const MIME_TYPE_KEY: &str = "MimeType";
pub const TERMINAL_KEY: &str = "Terminal";
pub const HIDDEN_KEY: &str = "Hidden";
pub const TRUE_VALUE: &str = "true";
pub const MIME_COMMAND: &str = "xdg-mime";
pub const MIME_FILETYPE_ARGUMENTS: [&str; 2] = ["query", "filetype"];
pub const MIME_DEFAULT_ARGUMENTS: [&str; 2] = ["query", "default"];
pub const DIRECTORY_MIME: &str = "inode/directory";
pub const TEXT_MIME_PREFIX: &str = "text/";
pub const PLAIN_TEXT_MIME: &str = "text/plain";
pub const OPEN_WITH_LIMIT: usize = 12;
pub const TERMINAL_VARIABLE: &str = "TERMINAL";
pub const PATH_VARIABLE: &str = "PATH";
pub const PATH_SEPARATOR: char = '/';
pub const EXECUTE_BITS: u32 = 0o111;
pub const DEFAULT_RUN_ARGUMENTS: &[&str] = &["-e"];

pub type KnownTerminal = (
    &'static [&'static str],
    &'static [&'static str],
    Option<&'static str>,
);

pub const KNOWN_TERMINALS: [KnownTerminal; 11] = [
    (&["xdg-terminal-exec"], &[], None),
    (&["kitty"], &[], Some("--directory=")),
    (&["foot"], &[], Some("--working-directory=")),
    (&["alacritty"], &["-e"], Some("--working-directory=")),
    (&["ghostty"], &["-e"], Some("--working-directory=")),
    (&["wezterm", "start"], &["--"], Some("--cwd=")),
    (&["gnome-terminal"], &["--"], Some("--working-directory=")),
    (&["konsole"], &["-e"], Some("--workdir=")),
    (&["xfce4-terminal"], &["-x"], Some("--working-directory=")),
    (&["kgx"], &["--"], Some("--working-directory=")),
    (&["xterm"], &["-e"], None),
];
