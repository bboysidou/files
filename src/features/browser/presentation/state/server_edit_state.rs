use gpui::{Entity, FocusHandle, Subscription};

use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::features::network::domain::entities::saved_server::SavedServer;
use crate::features::network::domain::entities::server_form_error::ServerFormError;
use crate::global_components::text_field::text_field_view::TextField;

pub struct ServerEditState {
    pub original: Option<SavedServer>,
    pub protocol: NetworkProtocol,
    pub fields: Vec<Entity<TextField>>,
    pub focus: FocusHandle,
    pub error: Option<ServerFormError>,
    pub _events: Vec<Subscription>,
}
