use gpui::{Hsla, hsla, rgb};

use crate::core::constants::hash::HUE_STEPS;
use crate::core::helpers::text::fnv1a::fnv1a;
use crate::core::theme::colors::{BADGE_LIGHTNESS, BADGE_SATURATION, FG_SOFT};

const OPAQUE: f32 = 1.0;

pub fn extension_color(extension: Option<&str>) -> Hsla {
    match extension.filter(|extension| !extension.is_empty()) {
        Some(extension) => {
            let hue = fnv1a(&extension.to_lowercase()) % HUE_STEPS;
            hsla(
                hue as f32 / HUE_STEPS as f32,
                BADGE_SATURATION,
                BADGE_LIGHTNESS,
                OPAQUE,
            )
        }
        None => rgb(FG_SOFT).into(),
    }
}
