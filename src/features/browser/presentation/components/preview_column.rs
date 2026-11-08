use gpui::{AnyElement, App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{CARD, LINE};
use crate::core::theme::metrics::{CARD_RADIUS, PREVIEW_COLUMN_WIDTH};
use crate::features::browser::presentation::components::preview_column_header::PreviewColumnHeader;
use crate::features::browser::presentation::components::quick_look_facts::QuickLookFacts;

#[derive(IntoElement)]
pub struct PreviewColumn {
    pub header: Option<PreviewColumnHeader>,
    pub body: AnyElement,
    pub facts: Option<QuickLookFacts>,
}

impl RenderOnce for PreviewColumn {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .w(px(PREVIEW_COLUMN_WIDTH))
            .h_full()
            .overflow_hidden()
            .rounded(px(CARD_RADIUS))
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(LINE))
            .children(self.header)
            .child(div().flex().flex_col().flex_1().min_h_0().child(self.body))
            .children(self.facts)
    }
}
