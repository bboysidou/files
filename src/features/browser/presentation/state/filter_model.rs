use gpui::{Entity, SharedString};

use crate::global_components::text_field::text_field_view::TextField;

pub struct FilterModel {
    pub field: Entity<TextField>,
    pub count: SharedString,
}
