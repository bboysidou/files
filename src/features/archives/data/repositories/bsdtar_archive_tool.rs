use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use crate::core::constants::preview::ARCHIVE_CREATE_ARGUMENTS;
use crate::core::constants::preview::{
    ARCHIVE_COMMAND, ARCHIVE_DESTINATION_FLAG, ARCHIVE_EXTRACT_ARGUMENTS, ARCHIVE_STAGING_NAME,
};
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;
use crate::features::archives::domain::repositories::archive_tool::ArchiveTool;
use crate::features::operations::domain::usecases::unique_target::unique_target;

pub struct BsdtarArchiveTool;

impl ArchiveTool for BsdtarArchiveTool {
    fn compress(&self, sources: &[PathBuf], archive: &Path) -> Result<(), String> {
        let parent = archive.parent().ok_or_else(String::new)?;
        let names: Vec<&OsStr> = sources
            .iter()
            .filter(|source| source.parent() == Some(parent))
            .filter_map(|source| source.file_name())
            .collect();
        if names.is_empty() || names.len() != sources.len() {
            return Err(String::new());
        }
        let access = SandboxAccess {
            read: Vec::new(),
            write: vec![parent.to_path_buf()],
        };
        let output = sandboxed_command(ARCHIVE_COMMAND, &access)
            .args(ARCHIVE_CREATE_ARGUMENTS)
            .arg(archive)
            .arg(ARCHIVE_DESTINATION_FLAG)
            .arg(parent)
            .args(names)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| error.to_string())?;
        if output.status.success() {
            return Ok(());
        }
        let _ = fs::remove_file(archive);
        Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
    }

    fn extract(&self, archive: &Path, destination: &Path) -> Result<(), String> {
        fs::create_dir(destination).map_err(|error| error.to_string())?;
        let access = SandboxAccess {
            read: vec![archive.to_path_buf()],
            write: vec![destination.to_path_buf()],
        };
        let output = sandboxed_command(ARCHIVE_COMMAND, &access)
            .args(ARCHIVE_EXTRACT_ARGUMENTS)
            .arg(archive)
            .arg(ARCHIVE_DESTINATION_FLAG)
            .arg(destination)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| error.to_string());
        let failure = match output {
            Ok(output) if output.status.success() => return flatten_single_folder(destination),
            Ok(output) => String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            Err(reason) => reason,
        };
        let _ = fs::remove_dir_all(destination);
        Err(failure)
    }
}

fn flatten_single_folder(destination: &Path) -> Result<(), String> {
    let mut children = fs::read_dir(destination).map_err(|error| error.to_string())?;
    let (Some(Ok(only)), None) = (children.next(), children.next()) else {
        return Ok(());
    };
    if !only.file_type().is_ok_and(|kind| kind.is_dir()) {
        return Ok(());
    }
    let Some(parent) = destination.parent() else {
        return Ok(());
    };
    let staging = unique_target(parent, ARCHIVE_STAGING_NAME, &|path| path.exists());
    fs::rename(destination, &staging).map_err(|error| error.to_string())?;
    fs::rename(staging.join(only.file_name()), destination).map_err(|error| error.to_string())?;
    fs::remove_dir(&staging).map_err(|error| error.to_string())
}
