use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, relative, rgb};

use crate::core::theme::colors::{ACCENT, LINE};
use crate::core::theme::metrics::JOB_BAR_HEIGHT;

#[derive(IntoElement)]
pub struct ProgressBar {
    pub fraction: f32,
}

impl RenderOnce for ProgressBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex_1()
            .h(px(JOB_BAR_HEIGHT))
            .rounded_full()
            .bg(rgb(LINE))
            .overflow_hidden()
            .child(
                div()
                    .h_full()
                    .w(relative(self.fraction))
                    .rounded_full()
                    .bg(rgb(ACCENT)),
            )
    }
}
