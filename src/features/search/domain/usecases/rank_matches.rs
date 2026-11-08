use std::path::PathBuf;

use super::match_score::match_score;
use crate::features::search::domain::entities::indexed_path::IndexedPath;

pub fn rank_matches(index: &[IndexedPath], query: &str, limit: usize) -> Vec<PathBuf> {
    let needle = query.trim().to_lowercase();
    if needle.is_empty() {
        return Vec::new();
    }
    let mut scored: Vec<(u32, usize, &IndexedPath)> = index
        .iter()
        .filter_map(|item| {
            let score = match_score(&item.name_lower, &needle)?;
            Some((score, item.path.as_os_str().len(), item))
        })
        .collect();
    scored.sort_by_key(|(score, length, _)| (*score, *length));
    scored
        .into_iter()
        .take(limit)
        .map(|(_, _, item)| item.path.clone())
        .collect()
}
