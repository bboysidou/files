use gpui::{Rgba, rgb, rgba};

use crate::core::theme::colors::{ACCENT, TRANSPARENT};

pub fn focus_ring(cursor: bool) -> Rgba {
    if cursor {
        rgb(ACCENT)
    } else {
        rgba(TRANSPARENT)
    }
}
