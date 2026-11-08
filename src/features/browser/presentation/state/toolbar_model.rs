use crate::features::places::domain::entities::crumb_trail::CrumbTrail;
use crate::global_components::handlers::{ClickHandler, IndexHandler};

pub struct ToolbarModel {
    pub trail: CrumbTrail,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub can_go_up: bool,
    pub on_back: ClickHandler,
    pub on_forward: ClickHandler,
    pub on_up: ClickHandler,
    pub on_crumb: IndexHandler,
}
