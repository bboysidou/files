use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::search::domain::entities::content_match::ContentMatch;
use crate::features::search::domain::entities::indexed_path::IndexedPath;

pub struct SearchState {
    pub root: PathBuf,
    pub query: String,
    pub index: Option<Arc<Vec<IndexedPath>>>,
    pub results: Vec<FileEntry>,
    pub return_focus: Option<PathBuf>,
    pub hits: HashMap<PathBuf, ContentMatch>,
    pub cancel: Arc<AtomicBool>,
}
