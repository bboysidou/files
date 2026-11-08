use std::time::Duration;

use gpui::Animation;

use crate::core::helpers::motion::cubic_bezier::cubic_bezier;
use crate::core::theme::metrics::{RAIL_COLLAPSED_WIDTH, RAIL_WIDTH};
use crate::core::theme::motion::{RAIL_TOGGLE_MS, STANDARD_EASING};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RailMotion {
    pub open: bool,
    pub generation: usize,
}

impl RailMotion {
    pub fn starting(open: bool) -> Self {
        Self {
            open,
            generation: 0,
        }
    }

    pub fn toggled(self) -> Self {
        Self {
            open: !self.open,
            generation: self.generation + 1,
        }
    }

    pub fn animating(self) -> bool {
        self.generation > 0
    }

    pub fn width(self) -> f32 {
        if self.open {
            RAIL_WIDTH
        } else {
            RAIL_COLLAPSED_WIDTH
        }
    }

    pub fn previous_width(self) -> f32 {
        if self.open {
            RAIL_COLLAPSED_WIDTH
        } else {
            RAIL_WIDTH
        }
    }

    pub fn animation(self) -> Animation {
        Animation::new(Duration::from_millis(RAIL_TOGGLE_MS))
            .with_easing(cubic_bezier(STANDARD_EASING))
    }
}
