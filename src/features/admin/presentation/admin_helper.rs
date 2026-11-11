use std::io::{self, BufRead, Read, Write};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use rustix::process::geteuid;

use crate::core::constants::admin::{ADMIN_IDLE_SECS, ADMIN_LINE_MAX_BYTES, ADMIN_PATHS_MAX};
use crate::core::enums::operations::job_phase::JobPhase;
use crate::features::admin::data::models::admin_reply::AdminReply;
use crate::features::admin::data::models::admin_request::AdminRequest;
use crate::features::admin::domain::usecases::is_safe_admin_path::is_safe_admin_path;
use crate::features::browser::data::repositories::local_directory_repository::LocalDirectoryRepository;
use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::operations::data::repositories::local_file_operations::LocalFileOperations;
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::repositories::file_operations::FileOperations;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

struct SilentSink;

impl ProgressSink for SilentSink {
    fn begin(&self, _total_bytes: u64) {}
    fn advance(&self, _bytes: u64) {}
    fn set_item(&self, _name: &str) {}
    fn set_phase(&self, _phase: JobPhase) {}
    fn is_cancelled(&self) -> bool {
        false
    }
}

fn read_lines(sender: Sender<String>) {
    let mut input = io::stdin().lock();
    loop {
        let mut line = String::new();
        match input
            .by_ref()
            .take(ADMIN_LINE_MAX_BYTES)
            .read_line(&mut line)
        {
            Ok(0) | Err(_) => return,
            Ok(_) if !line.ends_with('\n') => return,
            Ok(_) => {
                if sender.send(line).is_err() {
                    return;
                }
            }
        }
    }
}

fn outcome(result: Result<(), OperationError>) -> AdminReply {
    match result {
        Ok(()) => AdminReply::Done,
        Err(error) => AdminReply::Fault { error },
    }
}

fn query(request: AdminRequest) -> AdminReply {
    let (files, directories) = (LocalFileOperations, LocalDirectoryRepository);
    match request {
        AdminRequest::List { path } => match directories.list(&path) {
            Ok(entries) => AdminReply::Entries { entries },
            Err(_) => AdminReply::Fault {
                error: OperationError::Failed,
            },
        },
        AdminRequest::Entry { path } => AdminReply::Entry {
            entry: directories.entry(&path),
        },
        AdminRequest::Exists { path } => AdminReply::Flag {
            value: files.exists(&path),
        },
        AdminRequest::IsDir { path } => AdminReply::Flag {
            value: files.is_dir(&path),
        },
        AdminRequest::IsSameFile { first, second } => AdminReply::Flag {
            value: files.is_same_file(&first, &second),
        },
        AdminRequest::Children { path } => AdminReply::Paths {
            paths: files.children(&path),
        },
        AdminRequest::SizeOf { path } => AdminReply::Number {
            value: files.size_of(&path),
        },
        AdminRequest::HasFileOver { path, limit } => AdminReply::Flag {
            value: files.has_file_over(&path, limit),
        },
        AdminRequest::TransferBytes { from, to } => AdminReply::Number {
            value: files.transfer_bytes(&from, &to),
        },
        mutation => mutate(mutation),
    }
}

fn mutate(request: AdminRequest) -> AdminReply {
    let files = LocalFileOperations;
    outcome(match request {
        AdminRequest::Copy { from, to } => files.copy(&from, &to, &SilentSink),
        AdminRequest::Move { from, to } => files.move_path(&from, &to, &SilentSink),
        AdminRequest::Delete { paths } => files.delete(&paths),
        AdminRequest::CreateFolder { path } => files.create_folder(&path),
        AdminRequest::RemoveFolder { path } => files.remove_folder(&path),
        AdminRequest::CreateFile { path } => files.create_file(&path),
        AdminRequest::Link { target, link } => files.link(&target, &link),
        AdminRequest::SetMode { path, mode } => files.set_mode(&path, mode),
        _ => Err(OperationError::Failed),
    })
}

fn answer(line: &str) -> AdminReply {
    let refused = AdminReply::Fault {
        error: OperationError::Failed,
    };
    let Ok(request) = serde_json::from_str::<AdminRequest>(line) else {
        return refused;
    };
    let paths = request.paths();
    if paths.len() > ADMIN_PATHS_MAX || !paths.iter().all(|path| is_safe_admin_path(path)) {
        return refused;
    }
    query(request)
}

fn send(output: &mut impl Write, reply: &AdminReply) -> io::Result<()> {
    serde_json::to_writer(&mut *output, reply)?;
    output.write_all(b"\n")?;
    output.flush()
}

pub fn serve_admin_helper() -> i32 {
    if !geteuid().is_root() {
        return 1;
    }
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || read_lines(sender));
    let mut output = io::stdout().lock();
    if send(&mut output, &AdminReply::Ready).is_err() {
        return 1;
    }
    while let Ok(line) = receiver.recv_timeout(Duration::from_secs(ADMIN_IDLE_SECS)) {
        if send(&mut output, &answer(&line)).is_err() {
            break;
        }
    }
    0
}
