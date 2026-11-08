use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::constants::preview::{
    ARCHIVE_COMMAND, ARCHIVE_CREATE_ARGUMENTS, ARCHIVE_DESTINATION_FLAG, ARCHIVE_EXTRACT_ARGUMENTS,
    ARCHIVE_STAGING_NAME,
};
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;
use crate::features::archives::data::datasources::bsdtar_runner::{
    listed_total, run_verbose, tree_size,
};
use crate::features::archives::domain::entities::archive_error::ArchiveError;
use crate::features::archives::domain::repositories::archive_tool::ArchiveTool;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;
use crate::features::operations::domain::usecases::unique_target::unique_target;

pub struct BsdtarArchiveTool;

impl ArchiveTool for BsdtarArchiveTool {
    fn compress(
        &self,
        sources: &[PathBuf],
        archive: &Path,
        sink: &dyn ProgressSink,
    ) -> Result<(), ArchiveError> {
        let parent = archive
            .parent()
            .ok_or_else(|| ArchiveError::Failed(String::new()))?;
        let names: Vec<&OsStr> = sources
            .iter()
            .filter(|source| source.parent() == Some(parent))
            .filter_map(|source| source.file_name())
            .collect();
        if names.is_empty() || names.len() != sources.len() {
            return Err(ArchiveError::Failed(String::new()));
        }
        sink.begin(sources.iter().map(|source| tree_size(source)).sum());
        let access = SandboxAccess {
            read: Vec::new(),
            write: vec![parent.to_path_buf()],
        };
        let mut command = sandboxed_command(ARCHIVE_COMMAND, &access);
        command
            .args(ARCHIVE_CREATE_ARGUMENTS)
            .arg(archive)
            .arg(ARCHIVE_DESTINATION_FLAG)
            .arg(parent)
            .args(names);
        let outcome = run_verbose(command, parent, sink);
        if outcome.is_err() {
            let _ = fs::remove_file(archive);
        }
        outcome
    }

    fn extract(
        &self,
        archive: &Path,
        destination: &Path,
        sink: &dyn ProgressSink,
    ) -> Result<(), ArchiveError> {
        sink.begin(listed_total(archive));
        fs::create_dir(destination).map_err(|error| ArchiveError::Failed(error.to_string()))?;
        let access = SandboxAccess {
            read: vec![archive.to_path_buf()],
            write: vec![destination.to_path_buf()],
        };
        let mut command = sandboxed_command(ARCHIVE_COMMAND, &access);
        command
            .args(ARCHIVE_EXTRACT_ARGUMENTS)
            .arg(archive)
            .arg(ARCHIVE_DESTINATION_FLAG)
            .arg(destination);
        match run_verbose(command, destination, sink) {
            Ok(()) => flatten_single_folder(destination)
                .map_err(|error| ArchiveError::Failed(error.to_string())),
            Err(error) => {
                let _ = fs::remove_dir_all(destination);
                Err(error)
            }
        }
    }
}

fn flatten_single_folder(destination: &Path) -> std::io::Result<()> {
    let mut children = fs::read_dir(destination)?;
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
    fs::rename(destination, &staging)?;
    fs::rename(staging.join(only.file_name()), destination)?;
    fs::remove_dir(&staging)
}
