use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;

pub struct PaneStart {
    pub tab: TabSnapshot,
    pub options: ViewOptions,
}
