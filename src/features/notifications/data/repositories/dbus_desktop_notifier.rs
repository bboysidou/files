use std::collections::HashMap;
use std::sync::OnceLock;

use zbus::blocking::Connection;
use zbus::zvariant::Value;

use crate::core::constants::app::APP_ID;
use crate::core::constants::notifications::{
    CLOSE_METHOD, HINT_DESKTOP_ENTRY, HINT_RESIDENT, HINT_TAG, HINT_URGENCY, HINT_VALUE,
    NOTE_APP_ICON, NOTE_APP_NAME, NOTIFY_BUS_NAME, NOTIFY_BUS_PATH, NOTIFY_INTERFACE,
    NOTIFY_METHOD,
};
use crate::features::notifications::domain::entities::desktop_note::DesktopNote;
use crate::features::notifications::domain::repositories::desktop_notifier::DesktopNotifier;

#[derive(Default)]
pub struct DbusDesktopNotifier {
    connection: OnceLock<Option<Connection>>,
}

impl DbusDesktopNotifier {
    fn connection(&self) -> Option<&Connection> {
        self.connection
            .get_or_init(|| Connection::session().ok())
            .as_ref()
    }

    fn hints(note: &DesktopNote) -> HashMap<&'static str, Value<'_>> {
        let mut hints = HashMap::from([
            (HINT_URGENCY, Value::U8(note.urgency.code())),
            (HINT_DESKTOP_ENTRY, Value::from(APP_ID)),
        ]);
        if let Some(progress) = note.progress {
            hints.insert(HINT_VALUE, Value::I32(i32::from(progress)));
        }
        if let Some(tag) = &note.tag {
            hints.insert(HINT_TAG, Value::from(tag.as_str()));
        }
        if note.resident {
            hints.insert(HINT_RESIDENT, Value::Bool(true));
        }
        hints
    }
}

impl DesktopNotifier for DbusDesktopNotifier {
    fn show(&self, note: &DesktopNote) -> Option<u32> {
        let actions: Vec<&str> = Vec::new();
        let reply = self
            .connection()?
            .call_method(
                Some(NOTIFY_BUS_NAME),
                NOTIFY_BUS_PATH,
                Some(NOTIFY_INTERFACE),
                NOTIFY_METHOD,
                &(
                    NOTE_APP_NAME,
                    note.replaces,
                    NOTE_APP_ICON,
                    note.summary.as_str(),
                    note.body.as_str(),
                    actions,
                    Self::hints(note),
                    note.timeout_ms,
                ),
            )
            .ok()?;
        reply.body().deserialize::<u32>().ok()
    }

    fn close(&self, id: u32) {
        if let Some(connection) = self.connection() {
            let _ = connection.call_method(
                Some(NOTIFY_BUS_NAME),
                NOTIFY_BUS_PATH,
                Some(NOTIFY_INTERFACE),
                CLOSE_METHOD,
                &(id,),
            );
        }
    }
}
