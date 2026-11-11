use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::operations::domain::entities::operation_error::OperationError;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AdminReply {
    Ready,
    Done,
    Flag { value: bool },
    Number { value: u64 },
    Paths { paths: Vec<PathBuf> },
    Entries { entries: Vec<FileEntry> },
    Entry { entry: Option<FileEntry> },
    Fault { error: OperationError },
}
