use std::time::Duration;

use gpui::{
    Animation, AnimationExt, App, IntoElement, RenderOnce, Window, div, prelude::*, px, relative,
    rgb,
};

use crate::core::theme::colors::{ACCENT, LINE};
use crate::core::theme::metrics::{INDETERMINATE_SEGMENT_FRACTION, JOB_BAR_HEIGHT};
use crate::core::theme::motion::PROGRESS_ACTIVITY_MS;

#[derive(IntoElement)]
pub struct ProgressBar {
    pub fraction: Option<f32>,
}

impl RenderOnce for ProgressBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let fill = div().h_full().rounded_full().bg(rgb(ACCENT));
        let fill = match self.fraction.filter(|fraction| fraction.is_finite()) {
            Some(fraction) => fill
                .w(relative(fraction.clamp(0.0, 1.0)))
                .into_any_element(),
            None => fill
                .w(relative(INDETERMINATE_SEGMENT_FRACTION))
                .with_animation(
                    "progress-activity",
                    Animation::new(Duration::from_millis(PROGRESS_ACTIVITY_MS)).repeat(),
                    |fill, progress| {
                        fill.ml(relative(progress * (1.0 - INDETERMINATE_SEGMENT_FRACTION)))
                    },
                )
                .into_any_element(),
        };
        div()
            .flex_1()
            .h(px(JOB_BAR_HEIGHT))
            .rounded_full()
            .bg(rgb(LINE))
            .overflow_hidden()
            .child(fill)
    }
}
