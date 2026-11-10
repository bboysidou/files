use std::sync::{Arc, OnceLock};

use crate::features::notifications::data::repositories::dbus_desktop_notifier::DbusDesktopNotifier;
use crate::features::notifications::domain::repositories::desktop_notifier::DesktopNotifier;
use crate::features::notifications::domain::usecases::live_notes::LiveNotes;

static LIVE_NOTES: OnceLock<Arc<LiveNotes>> = OnceLock::new();

pub fn desktop_notifier() -> Arc<dyn DesktopNotifier> {
    Arc::new(DbusDesktopNotifier::default())
}

pub fn live_notes() -> Arc<LiveNotes> {
    LIVE_NOTES
        .get_or_init(|| Arc::new(LiveNotes::new(desktop_notifier())))
        .clone()
}
