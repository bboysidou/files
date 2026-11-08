use crate::core::constants::settings::PERMILLE;
use crate::core::theme::metrics::{DUAL_SPLIT_DEFAULT, DUAL_SPLIT_MAX, DUAL_SPLIT_MIN};
use crate::features::browser::presentation::state::tab::Tab;

pub struct DualPane {
    pub partner: Option<Tab>,
    pub focused: bool,
    pub split: f32,
}

impl DualPane {
    pub fn restored(permille: u16) -> Self {
        Self {
            split: (f32::from(permille) / PERMILLE).clamp(DUAL_SPLIT_MIN, DUAL_SPLIT_MAX),
            ..Self::default()
        }
    }
}

impl Default for DualPane {
    fn default() -> Self {
        Self {
            partner: None,
            focused: false,
            split: DUAL_SPLIT_DEFAULT,
        }
    }
}
