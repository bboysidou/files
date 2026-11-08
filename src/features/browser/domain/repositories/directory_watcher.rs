use std::path::Path;

use crate::features::browser::domain::entities::watch_subscription::WatchSubscription;

pub trait DirectoryWatcher: Send + Sync {
    fn watch(&self, directory: &Path) -> Option<WatchSubscription>;
}
