use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64};

pub enum ArchiveWatch {
    Reading(PathBuf),
    Writing {
        root: PathBuf,
        sources: Vec<PathBuf>,
        archive: PathBuf,
    },
}

pub struct ArchiveTracker {
    pub watch: ArchiveWatch,
    pub finished: AtomicU64,
    pub stopped: AtomicBool,
}

impl ArchiveTracker {
    pub fn new(watch: ArchiveWatch) -> Self {
        Self {
            watch,
            finished: AtomicU64::new(0),
            stopped: AtomicBool::new(false),
        }
    }
}
