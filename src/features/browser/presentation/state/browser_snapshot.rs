use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BrowserSnapshot {
    pub tabs: Vec<TabSnapshot>,
    pub active: usize,
    pub options: ViewOptions,
    pub preview_open: bool,
}
