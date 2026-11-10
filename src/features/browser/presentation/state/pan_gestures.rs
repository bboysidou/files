use std::cell::Cell;
use std::rc::Rc;

use gpui::{Pixels, Size};

use crate::global_components::handlers::{ClickHandler, PointHandler};

#[derive(Clone)]
pub struct PanGestures {
    pub stage: Rc<Cell<Size<Pixels>>>,
    pub on_press: PointHandler,
    pub on_drag: PointHandler,
    pub on_release: ClickHandler,
}
