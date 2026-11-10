use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Transfer {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl Transfer {
    fn swapped(&self) -> Self {
        Self {
            from: self.to.clone(),
            to: self.from.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ModeChange {
    pub path: PathBuf,
    pub from: u32,
    pub to: u32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FileOperation {
    Copy(Vec<Transfer>),
    Move(Vec<Transfer>),
    Rename(Transfer),
    Trash(Vec<PathBuf>),
    Restore(Vec<PathBuf>),
    CreateFolder(PathBuf),
    RemoveFolder(PathBuf),
    Extract(PathBuf),
    Compress(PathBuf),
    CreateFile(PathBuf),
    Link(Vec<Transfer>),
    SetMode(ModeChange),
    Batch(Vec<FileOperation>),
}

impl FileOperation {
    pub fn inverse(&self) -> Self {
        match self {
            FileOperation::Copy(transfers) => FileOperation::Trash(
                transfers
                    .iter()
                    .map(|transfer| transfer.to.clone())
                    .collect(),
            ),
            FileOperation::Move(transfers) => {
                FileOperation::Move(transfers.iter().rev().map(Transfer::swapped).collect())
            }
            FileOperation::Rename(transfer) => FileOperation::Rename(transfer.swapped()),
            FileOperation::Trash(paths) => FileOperation::Restore(paths.clone()),
            FileOperation::Restore(paths) => FileOperation::Trash(paths.clone()),
            FileOperation::CreateFolder(path) => FileOperation::RemoveFolder(path.clone()),
            FileOperation::RemoveFolder(path) => FileOperation::CreateFolder(path.clone()),
            FileOperation::Extract(path)
            | FileOperation::Compress(path)
            | FileOperation::CreateFile(path) => FileOperation::Trash(vec![path.clone()]),
            FileOperation::SetMode(change) => FileOperation::SetMode(ModeChange {
                path: change.path.clone(),
                from: change.to,
                to: change.from,
            }),
            FileOperation::Link(transfers) => FileOperation::Trash(
                transfers
                    .iter()
                    .map(|transfer| transfer.to.clone())
                    .collect(),
            ),
            FileOperation::Batch(steps) => {
                FileOperation::Batch(steps.iter().rev().map(FileOperation::inverse).collect())
            }
        }
    }

    pub fn touched_paths(&self) -> Vec<PathBuf> {
        match self {
            FileOperation::Copy(transfers)
            | FileOperation::Move(transfers)
            | FileOperation::Link(transfers) => transfers
                .iter()
                .flat_map(|transfer| [transfer.from.clone(), transfer.to.clone()])
                .collect(),
            FileOperation::Rename(transfer) => vec![transfer.from.clone(), transfer.to.clone()],
            FileOperation::Trash(paths) | FileOperation::Restore(paths) => paths.clone(),
            FileOperation::CreateFolder(path)
            | FileOperation::RemoveFolder(path)
            | FileOperation::Extract(path)
            | FileOperation::Compress(path)
            | FileOperation::CreateFile(path) => vec![path.clone()],
            FileOperation::SetMode(change) => vec![change.path.clone()],
            FileOperation::Batch(steps) => steps.iter().flat_map(Self::touched_paths).collect(),
        }
    }

    pub fn item_count(&self) -> usize {
        match self {
            FileOperation::Copy(transfers)
            | FileOperation::Move(transfers)
            | FileOperation::Link(transfers) => transfers.len(),
            FileOperation::Trash(paths) | FileOperation::Restore(paths) => paths.len(),
            FileOperation::Rename(_)
            | FileOperation::CreateFolder(_)
            | FileOperation::RemoveFolder(_)
            | FileOperation::Extract(_)
            | FileOperation::Compress(_)
            | FileOperation::CreateFile(_)
            | FileOperation::SetMode(_) => 1,
            FileOperation::Batch(steps) => steps.last().map_or(0, FileOperation::item_count),
        }
    }

    pub fn landing_path(&self) -> Option<PathBuf> {
        match self {
            FileOperation::Copy(transfers)
            | FileOperation::Move(transfers)
            | FileOperation::Link(transfers) => {
                transfers.first().map(|transfer| transfer.to.clone())
            }
            FileOperation::Rename(transfer) => Some(transfer.to.clone()),
            FileOperation::Restore(paths) => paths.first().cloned(),
            FileOperation::CreateFolder(path)
            | FileOperation::Extract(path)
            | FileOperation::Compress(path)
            | FileOperation::CreateFile(path) => Some(path.clone()),
            FileOperation::SetMode(change) => Some(change.path.clone()),
            FileOperation::Trash(_) | FileOperation::RemoveFolder(_) => None,
            FileOperation::Batch(steps) => steps.last().and_then(FileOperation::landing_path),
        }
    }
}
