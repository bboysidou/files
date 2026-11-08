use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::list_error::ListError;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LoadState {
    Loading,
    Ready(Vec<FileEntry>),
    Failed(ListError),
    Unlisted,
}
