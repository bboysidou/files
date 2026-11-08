use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::core::constants::search::{
    CONTENT_MAX_BYTES, CONTENT_RESULT_LIMIT, CONTENT_SNIFF_BYTES, CONTENT_SNIPPET_CHARS,
};
use crate::features::search::domain::entities::content_match::ContentMatch;
use crate::features::search::domain::entities::indexed_path::IndexedPath;
use crate::features::search::domain::repositories::content_searcher::ContentSearcher;

const NUL: u8 = 0;

pub struct LocalContentSearcher;

impl ContentSearcher for LocalContentSearcher {
    fn search(
        &self,
        paths: &[IndexedPath],
        needle: &str,
        cancel: &AtomicBool,
    ) -> Vec<ContentMatch> {
        let needle = needle.to_lowercase();
        let mut found = Vec::new();
        for indexed in paths {
            if cancel.load(Ordering::Relaxed) || found.len() >= CONTENT_RESULT_LIMIT {
                break;
            }
            if let Some(hit) = first_match(&indexed.path, &needle) {
                found.push(hit);
            }
        }
        found
    }
}

fn first_match(path: &Path, needle: &str) -> Option<ContentMatch> {
    let meta = path.metadata().ok()?;
    if !meta.is_file() || meta.len() > CONTENT_MAX_BYTES {
        return None;
    }
    let mut bytes = Vec::new();
    File::open(path).ok()?.read_to_end(&mut bytes).ok()?;
    if bytes
        .iter()
        .take(CONTENT_SNIFF_BYTES)
        .any(|byte| *byte == NUL)
    {
        return None;
    }
    let text = String::from_utf8_lossy(&bytes);
    text.lines().enumerate().find_map(|(index, line)| {
        line.to_lowercase().contains(needle).then(|| ContentMatch {
            path: path.to_path_buf(),
            line_number: index + 1,
            line: line.trim().chars().take(CONTENT_SNIPPET_CHARS).collect(),
        })
    })
}
