use gpui::{
    AnyElement, App, IntoElement, MouseButton, Pixels, RenderOnce, Window, div, prelude::*, px, rgb,
};

use crate::core::theme::colors::{CARD, LINE_STRONG, STAGE};
use crate::core::theme::metrics::QUICK_LOOK_RADIUS;
use crate::features::browser::presentation::components::quick_look_facts::QuickLookFacts;
use crate::features::browser::presentation::components::quick_look_header::QuickLookHeader;

#[derive(IntoElement)]
pub struct QuickLookPanel {
    pub header: QuickLookHeader,
    pub stage: AnyElement,
    pub facts: QuickLookFacts,
    pub size: (Pixels, Pixels),
}

impl RenderOnce for QuickLookPanel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(self.size.0)
            .h(self.size.1)
            .overflow_hidden()
            .rounded(px(QUICK_LOOK_RADIUS))
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(LINE_STRONG))
            .shadow_lg()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(self.header)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h_0()
                    .bg(rgb(STAGE))
                    .child(self.stage),
            )
            .child(self.facts)
    }
}
