use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, ChildStderr, Command, Stdio};
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use crate::core::constants::operations::ARCHIVE_POLL_MS;
use crate::core::constants::preview::ARCHIVE_VERBOSE_MARKS;
use crate::core::helpers::process::descendant_pids::descendant_pids;
use crate::core::helpers::process::open_file_position::open_file_position;
use crate::features::archives::domain::entities::archive_error::ArchiveError;
use crate::features::archives::domain::entities::archive_watch::{ArchiveTracker, ArchiveWatch};
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub fn run_verbose(
    mut command: Command,
    watch: ArchiveWatch,
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
    let tracker = ArchiveTracker::new(watch);
    let pid = child.id();
    let errors = thread::scope(|scope| {
        scope.spawn(|| monitor(pid, &tracker, sink));
        let errors = read_entries(stderr, &mut child, &tracker, sink);
        tracker.stopped.store(true, Ordering::Relaxed);
        errors
    })?;
    let status = child.wait().map_err(failed)?;
    if status.success() {
        Ok(())
    } else {
        Err(ArchiveError::Failed(errors.join("\n")))
    }
}

fn read_entries(
    stderr: ChildStderr,
    child: &mut Child,
    tracker: &ArchiveTracker,
    sink: &dyn ProgressSink,
) -> Result<Vec<String>, ArchiveError> {
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
                if let ArchiveWatch::Writing { root, .. } = &tracker.watch {
                    tracker
                        .finished
                        .fetch_add(file_size(&root.join(name)), Ordering::Relaxed);
                }
            }
            _ => errors.push(line),
        }
    }
    Ok(errors)
}

fn monitor(pid: u32, tracker: &ArchiveTracker, sink: &dyn ProgressSink) {
    let mut reported = 0;
    while !tracker.stopped.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(ARCHIVE_POLL_MS));
        let pids = descendant_pids(pid);
        let position = match &tracker.watch {
            ArchiveWatch::Reading(archive) => open_file_position(&pids, &|link| link == archive),
            ArchiveWatch::Writing {
                sources, archive, ..
            } => {
                let open = open_file_position(&pids, &|link| {
                    link != archive && sources.iter().any(|source| link.starts_with(source))
                });
                tracker.finished.load(Ordering::Relaxed) + open
            }
        };
        if position > reported {
            sink.advance(position - reported);
            reported = position;
        }
    }
}

pub fn file_size(path: &Path) -> u64 {
    fs::symlink_metadata(path)
        .ok()
        .filter(|meta| meta.is_file())
        .map_or(0, |meta| meta.len())
}
