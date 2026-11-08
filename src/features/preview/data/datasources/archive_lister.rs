use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Stdio;

use crate::core::constants::preview::{
    ARCHIVE_COMMAND, ARCHIVE_COUNT_LIMIT, ARCHIVE_DIR_SUFFIX, ARCHIVE_LIST_ARGUMENTS,
    ARCHIVE_SHOWN_MEMBERS,
};
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;
use crate::features::preview::domain::entities::archive_member::ArchiveMember;
use crate::features::preview::domain::entities::preview_content::PreviewContent;

pub fn list_archive(path: &Path) -> Option<PreviewContent> {
    let access = SandboxAccess {
        read: vec![path.to_path_buf()],
        write: Vec::new(),
    };
    let mut child = sandboxed_command(ARCHIVE_COMMAND, &access)
        .args(ARCHIVE_LIST_ARGUMENTS)
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let reader = BufReader::new(child.stdout.take()?);
    let mut members = Vec::new();
    let mut total = 0;
    for line in reader.lines().map_while(Result::ok) {
        total += 1;
        if members.len() < ARCHIVE_SHOWN_MEMBERS {
            members.push(member(&line));
        }
        if total >= ARCHIVE_COUNT_LIMIT {
            break;
        }
    }
    let capped = total >= ARCHIVE_COUNT_LIMIT;
    if capped {
        let _ = child.kill();
    }
    let finished = child.wait().ok()?;
    (capped || finished.success() || total > 0).then_some(PreviewContent::Archive {
        members,
        total,
        capped,
    })
}

fn member(line: &str) -> ArchiveMember {
    ArchiveMember {
        is_dir: line.ends_with(ARCHIVE_DIR_SUFFIX),
        path: line.trim_end_matches(ARCHIVE_DIR_SUFFIX).to_owned(),
    }
}
