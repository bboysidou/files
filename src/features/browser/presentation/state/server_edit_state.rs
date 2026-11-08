use gpui::{Entity, FocusHandle, Subscription};

use crate::features::network::domain::entities::saved_server::SavedServer;
use crate::global_components::text_field::text_field_view::TextField;

pub struct ServerEditState {
    pub original: SavedServer,
    pub fields: [Entity<TextField>; 2],
    pub focus: FocusHandle,
    pub _events: [Subscription; 2],
}
