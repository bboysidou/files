use crate::features::browser::domain::entities::watch_subscription::WatchSubscription;

pub trait MountWatcher: Send + Sync {
    fn watch(&self) -> Option<WatchSubscription>;
}
