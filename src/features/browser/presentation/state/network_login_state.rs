use gpui::{Entity, FocusHandle, Subscription, WeakEntity};

use crate::features::browser::presentation::pane_view::PaneView;
use crate::global_components::text_field::text_field_view::TextField;

pub struct NetworkLoginRequest {
    pub pane: WeakEntity<PaneView>,
    pub uri: String,
    pub rejected: bool,
}

pub struct NetworkLoginState {
    pub pane: WeakEntity<PaneView>,
    pub uri: String,
    pub rejected: bool,
    pub fields: Vec<Entity<TextField>>,
    pub focus: FocusHandle,
    pub _events: Vec<Subscription>,
}
