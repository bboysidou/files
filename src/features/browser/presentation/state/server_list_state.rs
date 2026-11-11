use gpui::{Entity, UniformListScrollHandle};

use crate::global_components::text_field::text_field_view::TextField;

pub struct ServerListState {
    pub filter: Entity<TextField>,
    pub cursor: usize,
    pub scroll: UniformListScrollHandle,
}
