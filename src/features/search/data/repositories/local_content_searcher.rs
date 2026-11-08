use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

use crate::core::constants::search::{
    CONTENT_LINE_CHARS, CONTENT_LINE_LIMIT, CONTENT_MAX_BYTES, CONTENT_MAX_WORKERS,
    CONTENT_RESULT_LIMIT, CONTENT_SNIFF_BYTES,
};
use crate::features::search::domain::entities::content_match::{ContentMatch, MatchLine};
use crate::features::search::domain::entities::indexed_path::IndexedPath;
use crate::features::search::domain::repositories::content_searcher::ContentSearcher;

const NUL: u8 = 0;

pub struct LocalContentSearcher;

struct Scan<'a> {
    paths: &'a [IndexedPath],
    needle: &'a str,
    cancel: &'a AtomicBool,
    next: AtomicUsize,
    found: Mutex<Vec<(usize, ContentMatch)>>,
}

impl ContentSearcher for LocalContentSearcher {
    fn search(
        &self,
        paths: &[IndexedPath],
        needle: &str,
        cancel: &AtomicBool,
    ) -> Vec<ContentMatch> {
        let needle = needle.to_lowercase();
        let scan = Scan {
            paths,
            needle: &needle,
            cancel,
            next: AtomicUsize::new(0),
            found: Mutex::new(Vec::new()),
        };
        let workers = thread::available_parallelism()
            .map_or(1, |count| count.get())
            .min(CONTENT_MAX_WORKERS);
        thread::scope(|scope| {
            for _ in 0..workers {
                scope.spawn(|| scan_worker(&scan));
            }
        });
        let mut found = scan.found.into_inner().unwrap_or_default();
        found.sort_by_key(|(index, _)| *index);
        found
            .into_iter()
            .take(CONTENT_RESULT_LIMIT)
            .map(|(_, hit)| hit)
            .collect()
    }
}

fn scan_worker(scan: &Scan) {
    loop {
        let index = scan.next.fetch_add(1, Ordering::Relaxed);
        let Some(indexed) = scan.paths.get(index) else {
            return;
        };
        let full = scan
            .found
            .lock()
            .map_or(true, |found| found.len() >= CONTENT_RESULT_LIMIT);
        if scan.cancel.load(Ordering::Relaxed) || full {
            return;
        }
        if let Some(hit) = file_match(&indexed.path, scan.needle)
            && let Ok(mut found) = scan.found.lock()
        {
            found.push((index, hit));
        }
    }
}

fn file_match(path: &Path, needle: &str) -> Option<ContentMatch> {
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
    let lowered = text.to_lowercase();
    let matches = lowered.matches(needle).count();
    if matches == 0 {
        return None;
    }
    let lines = text
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(needle))
        .take(CONTENT_LINE_LIMIT)
        .map(|(index, line)| MatchLine {
            number: index + 1,
            text: line.trim().chars().take(CONTENT_LINE_CHARS).collect(),
        })
        .collect();
    Some(ContentMatch {
        path: path.to_path_buf(),
        lines,
        matches,
    })
}
