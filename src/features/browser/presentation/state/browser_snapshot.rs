use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::places::domain::entities::place::PlaceTarget;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BrowserSnapshot {
    pub tabs: Vec<PlaceTarget>,
    pub active: usize,
    pub options: ViewOptions,
    pub preview_open: bool,
}
