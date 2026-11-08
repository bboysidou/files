use std::fs;
use std::path::{Path, PathBuf};

use crate::core::constants::files::HIDDEN_PREFIX;
use crate::core::constants::search::SEARCH_INDEX_LIMIT;
use crate::features::search::domain::entities::indexed_path::IndexedPath;
use crate::features::search::domain::repositories::search_indexer::SearchIndexer;

pub struct WalkSearchIndexer;

impl SearchIndexer for WalkSearchIndexer {
    fn index(&self, root: &Path, include_hidden: bool) -> Vec<IndexedPath> {
        let mut index = Vec::new();
        let mut pending: Vec<PathBuf> = vec![root.to_path_buf()];
        while let Some(directory) = pending.pop() {
            let Ok(items) = fs::read_dir(&directory) else {
                continue;
            };
            for item in items.filter_map(Result::ok) {
                if index.len() >= SEARCH_INDEX_LIMIT {
                    return index;
                }
                let name = item.file_name().to_string_lossy().into_owned();
                if !include_hidden && name.starts_with(HIDDEN_PREFIX) {
                    continue;
                }
                let is_real_dir = item.file_type().is_ok_and(|kind| kind.is_dir());
                if is_real_dir {
                    pending.push(item.path());
                }
                index.push(IndexedPath {
                    path: item.path(),
                    name_lower: name.to_lowercase(),
                });
            }
        }
        index
    }
}
