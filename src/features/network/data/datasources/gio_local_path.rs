use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::core::constants::network::{GIO_COMMAND, GIO_INFO, GIO_LOCALE, LOCAL_PATH_PREFIX};

pub fn gio_local_path(uri: &str) -> Option<PathBuf> {
    let info = Command::new(GIO_COMMAND)
        .args([GIO_INFO, uri])
        .env("LC_ALL", GIO_LOCALE)
        .env("LANGUAGE", GIO_LOCALE)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !info.status.success() {
        return None;
    }
    String::from_utf8_lossy(&info.stdout)
        .lines()
        .find_map(|line| line.strip_prefix(LOCAL_PATH_PREFIX))
        .map(|path| PathBuf::from(path.trim()))
}
