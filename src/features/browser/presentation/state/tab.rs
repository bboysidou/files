use gpui::{Entity, Subscription};

use crate::features::browser::presentation::pane_view::PaneView;

pub struct Tab {
    pub pane: Entity<PaneView>,
    pub _changes: Subscription,
    pub _events: Subscription,
}
