use gpui::SharedString;

use crate::global_components::handlers::{ClickHandler, IndexHandler};

pub struct TabStripModel {
    pub titles: Vec<SharedString>,
    pub active: usize,
    pub on_select: IndexHandler,
    pub on_close: IndexHandler,
    pub on_new: ClickHandler,
}
