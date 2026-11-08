use std::path::PathBuf;
use std::time::SystemTime;

use crate::core::enums::files::file_kind::FileKind;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiskUsage {
    pub mount: PathBuf,
    pub total: u64,
    pub used: u64,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KindUsage {
    pub kind: FileKind,
    pub bytes: u64,
    pub count: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RecentFile {
    pub path: PathBuf,
    pub modified: SystemTime,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct OverviewReport {
    pub disks: Vec<DiskUsage>,
    pub kinds: Vec<KindUsage>,
    pub scanned: usize,
    pub recent: Vec<RecentFile>,
}
