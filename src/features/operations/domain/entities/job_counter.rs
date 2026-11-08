use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

#[derive(Debug, Default)]
pub struct JobCounter {
    total: AtomicU64,
    done: AtomicU64,
    cancelled: AtomicBool,
    item: Mutex<String>,
}

impl JobCounter {
    pub fn total(&self) -> u64 {
        self.total.load(Ordering::Relaxed)
    }

    pub fn done(&self) -> u64 {
        self.done.load(Ordering::Relaxed)
    }

    pub fn item(&self) -> String {
        self.item
            .lock()
            .map(|item| item.clone())
            .unwrap_or_default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
}

impl ProgressSink for JobCounter {
    fn begin(&self, total_bytes: u64) {
        self.total.store(total_bytes, Ordering::Relaxed);
    }

    fn advance(&self, bytes: u64) {
        self.done.fetch_add(bytes, Ordering::Relaxed);
    }

    fn set_item(&self, name: &str) {
        if let Ok(mut item) = self.item.lock() {
            name.clone_into(&mut item);
        }
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}
