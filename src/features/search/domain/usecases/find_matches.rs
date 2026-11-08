use std::path::{Path, PathBuf};

use super::rank_matches::rank_matches;
use crate::core::enums::tags::tag_color::TagColor;
use crate::features::search::domain::entities::indexed_path::IndexedPath;
use crate::features::tags::domain::usecases::parse_tag_query::parse_tag_query;

pub fn find_matches(
    index: &[IndexedPath],
    query: &str,
    limit: usize,
    tags_of: &dyn Fn(&Path) -> Vec<TagColor>,
) -> Vec<PathBuf> {
    let (tag, rest) = parse_tag_query(query);
    let Some(tag) = tag else {
        return rank_matches(index, &rest, limit);
    };
    let tagged: Vec<IndexedPath> = index
        .iter()
        .filter(|item| tags_of(&item.path).contains(&tag))
        .cloned()
        .collect();
    if rest.is_empty() {
        return tagged
            .into_iter()
            .take(limit)
            .map(|item| item.path)
            .collect();
    }
    rank_matches(&tagged, &rest, limit)
}
