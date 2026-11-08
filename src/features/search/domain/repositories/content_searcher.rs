use std::sync::atomic::AtomicBool;

use crate::features::search::domain::entities::content_match::ContentMatch;
use crate::features::search::domain::entities::indexed_path::IndexedPath;

pub trait ContentSearcher: Send + Sync {
    fn search(&self, paths: &[IndexedPath], needle: &str, cancel: &AtomicBool)
    -> Vec<ContentMatch>;
}
