use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::core::constants::preview::{
    ARCHIVE_COMMAND, ARCHIVE_LIST_SIZE_FIELD, ARCHIVE_VERBOSE_LIST_ARGUMENTS, ARCHIVE_VERBOSE_MARKS,
};
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;
use crate::features::archives::domain::entities::archive_error::ArchiveError;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub fn run_verbose(
    mut command: Command,
    root: &Path,
    sink: &dyn ProgressSink,
) -> Result<(), ArchiveError> {
    let failed = |error: std::io::Error| ArchiveError::Failed(error.to_string());
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(failed)?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| ArchiveError::Failed(String::new()))?;
    let mut errors = Vec::new();
    for line in BufReader::new(stderr).lines().map_while(Result::ok) {
        if sink.is_cancelled() {
            let _ = child.kill();
            let _ = child.wait();
            return Err(ArchiveError::Cancelled);
        }
        match line.split_once(' ') {
            Some((mark, name)) if ARCHIVE_VERBOSE_MARKS.contains(&mark) => {
                sink.set_item(name);
                sink.advance(file_size(&root.join(name)));
            }
            _ => errors.push(line),
        }
    }
    let status = child.wait().map_err(failed)?;
    if status.success() {
        Ok(())
    } else {
        Err(ArchiveError::Failed(errors.join("\n")))
    }
}

pub fn listed_total(archive: &Path) -> u64 {
    let access = SandboxAccess {
        read: vec![archive.to_path_buf()],
        write: Vec::new(),
    };
    let Ok(output) = sandboxed_command(ARCHIVE_COMMAND, &access)
        .args(ARCHIVE_VERBOSE_LIST_ARGUMENTS)
        .arg(archive)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
    else {
        return 0;
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().nth(ARCHIVE_LIST_SIZE_FIELD))
        .filter_map(|size| size.parse::<u64>().ok())
        .sum()
}

pub fn tree_size(path: &Path) -> u64 {
    let Ok(meta) = fs::symlink_metadata(path) else {
        return 0;
    };
    if !meta.is_dir() {
        return meta.len();
    }
    fs::read_dir(path)
        .map(|listing| listing.flatten().map(|item| tree_size(&item.path())).sum())
        .unwrap_or_default()
}

fn file_size(path: &Path) -> u64 {
    fs::symlink_metadata(path)
        .ok()
        .filter(|meta| meta.is_file())
        .map_or(0, |meta| meta.len())
}
