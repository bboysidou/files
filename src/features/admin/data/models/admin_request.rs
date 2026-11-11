use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "verb", rename_all = "snake_case", deny_unknown_fields)]
pub enum AdminRequest {
    List { path: PathBuf },
    Entry { path: PathBuf },
    Exists { path: PathBuf },
    IsDir { path: PathBuf },
    IsSameFile { first: PathBuf, second: PathBuf },
    Children { path: PathBuf },
    SizeOf { path: PathBuf },
    HasFileOver { path: PathBuf, limit: u64 },
    TransferBytes { from: PathBuf, to: PathBuf },
    Copy { from: PathBuf, to: PathBuf },
    Move { from: PathBuf, to: PathBuf },
    Delete { paths: Vec<PathBuf> },
    CreateFolder { path: PathBuf },
    RemoveFolder { path: PathBuf },
    CreateFile { path: PathBuf },
    Link { target: PathBuf, link: PathBuf },
    SetMode { path: PathBuf, mode: u32 },
}

impl AdminRequest {
    pub fn paths(&self) -> Vec<&PathBuf> {
        match self {
            Self::List { path }
            | Self::Entry { path }
            | Self::Exists { path }
            | Self::IsDir { path }
            | Self::Children { path }
            | Self::SizeOf { path }
            | Self::HasFileOver { path, .. }
            | Self::CreateFolder { path }
            | Self::RemoveFolder { path }
            | Self::CreateFile { path }
            | Self::SetMode { path, .. } => vec![path],
            Self::IsSameFile {
                first: from,
                second: to,
            }
            | Self::TransferBytes { from, to }
            | Self::Copy { from, to }
            | Self::Move { from, to }
            | Self::Link {
                target: from,
                link: to,
            } => vec![from, to],
            Self::Delete { paths } => paths.iter().collect(),
        }
    }
}
