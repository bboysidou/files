use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::constants::applications::{
    APPLICATIONS_DIR, DATA_DIRS_SEPARATOR, DATA_DIRS_VARIABLE, DEFAULT_DATA_DIRS,
    DESKTOP_EXTENSION, DESKTOP_ID_SEPARATOR,
};
use crate::features::applications::data::datasources::desktop_entry_parser::parse_desktop_entry;
use crate::features::applications::data::models::desktop_entry::DesktopEntry;

struct Scan {
    seen: HashSet<String>,
    entries: Vec<DesktopEntry>,
}

pub fn scan_desktop_entries() -> Vec<DesktopEntry> {
    let mut scan = Scan {
        seen: HashSet::new(),
        entries: Vec::new(),
    };
    for root in application_dirs() {
        collect(&root, &root, &mut scan);
    }
    scan.entries
}

fn application_dirs() -> Vec<PathBuf> {
    let data_dirs = env::var(DATA_DIRS_VARIABLE)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_DATA_DIRS.to_owned());
    dirs::data_dir()
        .into_iter()
        .chain(data_dirs.split(DATA_DIRS_SEPARATOR).map(PathBuf::from))
        .map(|dir| dir.join(APPLICATIONS_DIR))
        .collect()
}

fn collect(root: &Path, dir: &Path, scan: &mut Scan) {
    let Ok(listing) = fs::read_dir(dir) else {
        return;
    };
    for item in listing.flatten() {
        let path = item.path();
        if item.file_type().is_ok_and(|kind| kind.is_dir()) {
            collect(root, &path, scan);
        } else if path.extension().is_some_and(|ext| ext == DESKTOP_EXTENSION) {
            read_entry(root, &path, scan);
        }
    }
}

fn read_entry(root: &Path, path: &Path, scan: &mut Scan) {
    let Some(id) = desktop_id(root, path) else {
        return;
    };
    if !scan.seen.insert(id.clone()) {
        return;
    }
    if let Some(entry) = fs::read_to_string(path)
        .ok()
        .and_then(|text| parse_desktop_entry(id, &text))
    {
        scan.entries.push(entry);
    }
}

fn desktop_id(root: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(root).ok()?;
    let parts: Vec<&str> = relative
        .components()
        .map(|part| part.as_os_str().to_str())
        .collect::<Option<_>>()?;
    Some(parts.join(DESKTOP_ID_SEPARATOR))
}
