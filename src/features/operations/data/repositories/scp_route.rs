use std::fs;
use std::io;
use std::path::Path;

use crate::core::constants::network::SCP_USER_SEPARATOR;
use crate::core::constants::ssh::{SCP_GLOB_CHARS, SCP_GLOB_ESCAPE, SCP_REMOTE_SEPARATOR};
use crate::features::network::domain::entities::sftp_location::SftpLocation;
use crate::features::network::domain::usecases::sftp_location_of::sftp_location_of;
use crate::features::operations::data::datasources::scp_session::scp_session;
use crate::features::operations::data::datasources::sftp_probe::sftp_reachable;
use crate::features::operations::domain::entities::scp_job::ScpJob;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub fn scp_route(from: &Path, to: &Path, sink: &dyn ProgressSink) -> Option<io::Result<()>> {
    let (remote, job) = match (sftp_location_of(from), sftp_location_of(to)) {
        (Some(remote), None) => {
            let source = remote_spec(&remote, escape_glob(&remote.remote_path));
            let job = local_job(source, to.to_string_lossy().into_owned(), to, to);
            (remote, job)
        }
        (None, Some(remote)) => {
            let target = remote_spec(&remote, remote.remote_path.clone());
            let job = local_job(from.to_string_lossy().into_owned(), target, from, to);
            (remote, job)
        }
        _ => return None,
    };
    if !sftp_reachable(&destination(&remote), remote.port.as_deref()) {
        return None;
    }
    let job = ScpJob {
        port: remote.port,
        ..job
    };
    let result = scp_session(&job, sink);
    if result.is_err() {
        let _ = remove_partial(&job.partial);
    }
    Some(result)
}

fn local_job(source: String, target: String, local: &Path, partial: &Path) -> ScpJob {
    ScpJob {
        source,
        target,
        port: None,
        local: local.to_path_buf(),
        partial: partial.to_path_buf(),
    }
}

fn destination(remote: &SftpLocation) -> String {
    match &remote.user {
        Some(user) => format!("{user}{SCP_USER_SEPARATOR}{}", remote.host),
        None => remote.host.clone(),
    }
}

fn remote_spec(remote: &SftpLocation, path: String) -> String {
    format!("{}{SCP_REMOTE_SEPARATOR}{path}", destination(remote))
}

fn escape_glob(path: &str) -> String {
    let mut escaped = String::with_capacity(path.len());
    for found in path.chars() {
        if SCP_GLOB_CHARS.contains(&found) {
            escaped.push(SCP_GLOB_ESCAPE);
        }
        escaped.push(found);
    }
    escaped
}

fn remove_partial(path: &Path) -> io::Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}
