use gpui::SharedString;

use crate::global_components::handlers::{
    ClickHandler, IndexHandler, IndexPairHandler, IndexPathHandler,
};

pub struct TabStripModel {
    pub titles: Vec<SharedString>,
    pub active: usize,
    pub on_select: IndexHandler,
    pub on_close: IndexHandler,
    pub on_move: IndexPairHandler,
    pub on_tear: IndexPathHandler,
    pub on_drag_end: ClickHandler,
    pub on_drag_home: ClickHandler,
    pub on_new: ClickHandler,
}
