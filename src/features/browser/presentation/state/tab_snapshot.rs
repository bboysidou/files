use crate::features::browser::domain::entities::navigation_history::NavigationHistory;
use crate::features::places::domain::entities::place::PlaceTarget;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TabSnapshot {
    pub location: PlaceTarget,
    pub history: NavigationHistory,
}

impl TabSnapshot {
    pub fn fresh(location: PlaceTarget) -> Self {
        Self {
            location,
            history: NavigationHistory::default(),
        }
    }
}
