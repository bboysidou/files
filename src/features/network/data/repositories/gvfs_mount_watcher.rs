use std::thread;

use futures::channel::mpsc;
use zbus::MatchRule;
use zbus::blocking::{Connection, MessageIterator};
use zbus::message::Type;

use crate::core::constants::network::{GVFS_TRACKER_INTERFACE, GVFS_TRACKER_PATH};
use crate::features::browser::domain::entities::watch_subscription::WatchSubscription;
use crate::features::network::domain::repositories::mount_watcher::MountWatcher;

pub struct GvfsMountWatcher;

impl MountWatcher for GvfsMountWatcher {
    fn watch(&self) -> Option<WatchSubscription> {
        let connection = Connection::session().ok()?;
        let rule = MatchRule::builder()
            .msg_type(Type::Signal)
            .interface(GVFS_TRACKER_INTERFACE)
            .ok()?
            .path(GVFS_TRACKER_PATH)
            .ok()?
            .build();
        let messages = MessageIterator::for_match_rule(rule, &connection, None).ok()?;
        let (sender, changes) = mpsc::unbounded();
        thread::spawn(move || {
            for message in messages {
                if message.is_err() || sender.unbounded_send(()).is_err() {
                    break;
                }
            }
        });
        Some(WatchSubscription {
            changes,
            guard: Box::new(connection),
        })
    }
}
