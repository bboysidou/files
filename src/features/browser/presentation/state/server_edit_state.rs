use gpui::{Entity, FocusHandle, Subscription};

use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::core::enums::network::server_tab::ServerTab;
use crate::features::browser::presentation::state::server_list_state::ServerListState;
use crate::features::network::domain::entities::saved_server::SavedServer;
use crate::features::network::domain::entities::server_form_error::ServerFormError;
use crate::features::network::domain::entities::ssh_host::SshHost;
use crate::global_components::text_field::text_field_view::TextField;

pub struct ServerEditState {
    pub original: Option<SavedServer>,
    pub protocol: NetworkProtocol,
    pub fields: Vec<Entity<TextField>>,
    pub focus: FocusHandle,
    pub error: Option<ServerFormError>,
    pub ssh_hosts: Vec<SshHost>,
    pub tab: ServerTab,
    pub list: ServerListState,
    pub _events: Vec<Subscription>,
}
