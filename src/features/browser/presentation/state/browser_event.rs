use crate::features::browser::presentation::state::server_edit::PendingEdit;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::network::domain::entities::saved_server::SavedServer;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BrowserEvent {
    ServerConnected {
        uri: String,
        edit: Option<PendingEdit>,
    },
    ServerRenamed(SavedServer),
    TabDetached(TabSnapshot),
}
