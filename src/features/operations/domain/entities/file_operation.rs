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
pub enum FileOperation {
    Copy(Vec<Transfer>),
    Move(Vec<Transfer>),
    Rename(Transfer),
    Trash(Vec<PathBuf>),
    Restore(Vec<PathBuf>),
    CreateFolder(PathBuf),
    RemoveFolder(PathBuf),
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
        }
    }

    pub fn item_count(&self) -> usize {
        match self {
            FileOperation::Copy(transfers) | FileOperation::Move(transfers) => transfers.len(),
            FileOperation::Trash(paths) | FileOperation::Restore(paths) => paths.len(),
            FileOperation::Rename(_)
            | FileOperation::CreateFolder(_)
            | FileOperation::RemoveFolder(_) => 1,
        }
    }

    pub fn landing_path(&self) -> Option<PathBuf> {
        match self {
            FileOperation::Copy(transfers) | FileOperation::Move(transfers) => {
                transfers.first().map(|transfer| transfer.to.clone())
            }
            FileOperation::Rename(transfer) => Some(transfer.to.clone()),
            FileOperation::Restore(paths) => paths.first().cloned(),
            FileOperation::CreateFolder(path) => Some(path.clone()),
            FileOperation::Trash(_) | FileOperation::RemoveFolder(_) => None,
        }
    }
}
