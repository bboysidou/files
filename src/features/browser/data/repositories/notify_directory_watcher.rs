use std::path::Path;

use futures::channel::mpsc;
use notify::event::EventKind;
use notify::{Event, RecursiveMode, Watcher};

use crate::features::browser::domain::entities::watch_subscription::WatchSubscription;
use crate::features::browser::domain::repositories::directory_watcher::DirectoryWatcher;

pub struct NotifyDirectoryWatcher;

impl NotifyDirectoryWatcher {
    fn is_change(event: &Event) -> bool {
        !matches!(event.kind, EventKind::Access(_))
    }
}

impl DirectoryWatcher for NotifyDirectoryWatcher {
    fn watch(&self, directory: &Path) -> Option<WatchSubscription> {
        let (sender, changes) = mpsc::unbounded();
        let mut watcher = notify::recommended_watcher(move |result: notify::Result<Event>| {
            if result.is_ok_and(|event| Self::is_change(&event)) {
                let _ = sender.unbounded_send(());
            }
        })
        .ok()?;
        watcher.watch(directory, RecursiveMode::NonRecursive).ok()?;
        Some(WatchSubscription {
            changes,
            guard: Box::new(watcher),
        })
    }
}
