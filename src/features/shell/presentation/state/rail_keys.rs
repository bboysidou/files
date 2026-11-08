use gpui::FocusHandle;

use crate::features::shell::presentation::state::rail_stop::RailStop;

pub struct RailKeys {
    pub focus: FocusHandle,
    pub cursor: RailStop,
}
