use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::places::domain::entities::place::PlaceTarget;

pub struct PaneStart {
    pub target: PlaceTarget,
    pub options: ViewOptions,
}
