use gpui::{Entity, Subscription};

use crate::global_components::text_field::text_field_view::TextField;

pub struct PaneInputs {
    pub filter: Entity<TextField>,
    pub rename: Entity<TextField>,
    pub search: Entity<TextField>,
    pub path: Entity<TextField>,
    pub subscriptions: Vec<Subscription>,
}
