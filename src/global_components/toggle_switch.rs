use gpui::{App, ElementId, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{ACCENT, FG_BRIGHT, LINE_STRONG};
use crate::core::theme::metrics::{TOGGLE_HEIGHT, TOGGLE_KNOB, TOGGLE_PAD, TOGGLE_WIDTH};
use crate::global_components::handlers::ClickHandler;

#[derive(IntoElement)]
pub struct ToggleSwitch {
    pub id: ElementId,
    pub on: bool,
    pub on_toggle: ClickHandler,
}

impl RenderOnce for ToggleSwitch {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_toggle = self.on_toggle;
        div()
            .id(self.id)
            .flex()
            .flex_shrink_0()
            .items_center()
            .when(self.on, |track| track.justify_end())
            .w(px(TOGGLE_WIDTH))
            .h(px(TOGGLE_HEIGHT))
            .p(px(TOGGLE_PAD))
            .rounded_full()
            .bg(rgb(if self.on { ACCENT } else { LINE_STRONG }))
            .cursor_pointer()
            .on_click(move |_, window, cx| on_toggle(window, cx))
            .child(
                div()
                    .size(px(TOGGLE_KNOB))
                    .rounded_full()
                    .bg(rgb(FG_BRIGHT)),
            )
    }
}
