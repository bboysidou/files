use crate::features::browser::presentation::state::filter_model::FilterModel;
use crate::features::places::domain::entities::crumb_trail::CrumbTrail;
use crate::global_components::handlers::{ClickHandler, IndexHandler};

pub struct ToolbarModel {
    pub trail: CrumbTrail,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub can_go_up: bool,
    pub filter: Option<FilterModel>,
    pub on_back: ClickHandler,
    pub on_forward: ClickHandler,
    pub on_up: ClickHandler,
    pub on_filter: ClickHandler,
    pub on_crumb: IndexHandler,
}
