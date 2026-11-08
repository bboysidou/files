use gpui::Entity;

use crate::global_components::text_field::text_field_view::TextField;

pub struct FilterModel {
    pub field: Entity<TextField>,
    pub shown: usize,
    pub total: usize,
}
