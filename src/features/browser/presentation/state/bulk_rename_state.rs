use std::path::PathBuf;

use gpui::{Entity, FocusHandle, Subscription};

use crate::global_components::text_field::text_field_view::TextField;

pub struct BulkRenameState {
    pub sources: Vec<PathBuf>,
    pub find: Entity<TextField>,
    pub replace: Entity<TextField>,
    pub focus: FocusHandle,
    pub _events: [Subscription; 2],
}

pub struct BulkPlan {
    pub old: Vec<String>,
    pub new: Vec<String>,
    pub conflicts: Vec<bool>,
}
