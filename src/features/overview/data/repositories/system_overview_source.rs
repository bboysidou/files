use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use jiff::Timestamp;

use crate::core::constants::files::{
    DF_ARGUMENTS, DF_COMMAND, HIDDEN_PREFIX, OVERVIEW_RECENT_LIMIT, OVERVIEW_WALK_LIMIT,
    RECENT_FILES_NAME,
};
use crate::core::enums::files::file_kind::FileKind;
use crate::core::helpers::files::path_from_uri::path_from_uri;
use crate::features::overview::domain::entities::overview_report::{
    DiskUsage, KindUsage, OverviewReport, RecentFile,
};
use crate::features::overview::domain::repositories::overview_source::OverviewSource;

const HREF_ATTRIBUTE: &str = "href=\"";
const MODIFIED_ATTRIBUTE: &str = "modified=\"";
const BOOKMARK_TAG: &str = "<bookmark ";
const QUOTE: char = '"';

pub struct SystemOverviewSource;

fn disk_row(line: &str) -> Option<(String, DiskUsage)> {
    let mut fields = line.split_whitespace();
    let source = fields.next()?.to_string();
    let mount = PathBuf::from(fields.next()?);
    let total = fields.next()?.parse().ok()?;
    let used = fields.next()?.parse().ok()?;
    Some((source, DiskUsage { mount, total, used }))
}

fn disks() -> Vec<DiskUsage> {
    let Ok(output) = Command::new(DF_COMMAND).args(DF_ARGUMENTS).output() else {
        return Vec::new();
    };
    let mut by_source: HashMap<String, DiskUsage> = HashMap::new();
    for (source, usage) in String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1)
        .filter_map(disk_row)
    {
        let shorter = by_source
            .get(&source)
            .is_none_or(|known| usage.mount.as_os_str().len() < known.mount.as_os_str().len());
        if shorter && usage.total > 0 {
            by_source.insert(source, usage);
        }
    }
    let mut disks: Vec<DiskUsage> = by_source.into_values().collect();
    disks.sort_by(|left, right| left.mount.cmp(&right.mount));
    disks
}

fn walk(home: &Path) -> (Vec<KindUsage>, usize) {
    let mut totals: HashMap<FileKind, (u64, usize)> = HashMap::new();
    let (mut pending, mut scanned) = (vec![home.to_path_buf()], 0);
    while let Some(folder) = pending.pop() {
        let Ok(items) = fs::read_dir(&folder) else {
            continue;
        };
        for item in items.flatten() {
            if scanned >= OVERVIEW_WALK_LIMIT
                || item
                    .file_name()
                    .to_string_lossy()
                    .starts_with(HIDDEN_PREFIX)
            {
                continue;
            }
            let Ok(kind) = item.file_type() else {
                continue;
            };
            if kind.is_dir() {
                pending.push(item.path());
            } else if kind.is_file() {
                scanned += 1;
                let path = item.path();
                let extension = path
                    .extension()
                    .map(|extension| extension.to_string_lossy().into_owned());
                let size = item.metadata().map(|meta| meta.len()).unwrap_or_default();
                let entry = totals
                    .entry(FileKind::classify(extension.as_deref(), false))
                    .or_default();
                *entry = (entry.0 + size, entry.1 + 1);
            }
        }
    }
    let mut kinds: Vec<KindUsage> = totals
        .into_iter()
        .map(|(kind, (bytes, count))| KindUsage { kind, bytes, count })
        .collect();
    kinds.sort_by_key(|usage| std::cmp::Reverse(usage.bytes));
    (kinds, scanned)
}

fn attribute<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let start = text.find(name)? + name.len();
    let rest = &text[start..];
    Some(&rest[..rest.find(QUOTE)?])
}

fn recent() -> Vec<RecentFile> {
    let Some(file) = dirs::data_dir().map(|data| data.join(RECENT_FILES_NAME)) else {
        return Vec::new();
    };
    let text = fs::read_to_string(file).unwrap_or_default();
    let mut found: Vec<RecentFile> = text
        .split(BOOKMARK_TAG)
        .skip(1)
        .filter_map(|chunk| {
            let header = &chunk[..chunk.find('>')?];
            let path = path_from_uri(attribute(header, HREF_ATTRIBUTE)?)?;
            let stamp: Timestamp = attribute(header, MODIFIED_ATTRIBUTE)?.parse().ok()?;
            let modified = SystemTime::from(stamp);
            path.is_file().then_some(RecentFile { path, modified })
        })
        .collect();
    found.sort_by_key(|file| std::cmp::Reverse(file.modified));
    found.truncate(OVERVIEW_RECENT_LIMIT);
    found
}

impl OverviewSource for SystemOverviewSource {
    fn report(&self, home: &Path) -> OverviewReport {
        let (kinds, scanned) = walk(home);
        OverviewReport {
            disks: disks(),
            kinds,
            scanned,
            recent: recent(),
        }
    }
}
