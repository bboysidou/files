use std::path::Path;

use crate::features::search::domain::entities::indexed_path::IndexedPath;

pub trait SearchIndexer: Send + Sync {
    fn index(&self, root: &Path, include_hidden: bool) -> Vec<IndexedPath>;
}
