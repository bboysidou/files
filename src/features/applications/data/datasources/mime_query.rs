use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::core::constants::applications::{
    DIRECTORY_MIME, MIME_COMMAND, MIME_DEFAULT_ARGUMENTS, MIME_FILETYPE_ARGUMENTS,
};

pub fn file_mime(path: &Path) -> Option<String> {
    if path.is_dir() {
        return Some(DIRECTORY_MIME.to_owned());
    }
    run_query(MIME_FILETYPE_ARGUMENTS, path.as_os_str())
}

pub fn default_app_id(mime: &str) -> Option<String> {
    run_query(MIME_DEFAULT_ARGUMENTS, OsStr::new(mime))
}

fn run_query(arguments: [&str; 2], subject: &OsStr) -> Option<String> {
    let output = Command::new(MIME_COMMAND)
        .args(arguments)
        .arg(subject)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())?;
    let answer = String::from_utf8(output.stdout).ok()?.trim().to_owned();
    (!answer.is_empty()).then_some(answer)
}
