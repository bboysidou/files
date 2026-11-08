use gpui::{
    App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, relative, rgb,
};

use crate::core::theme::colors::{FG_BRIGHT, FG_MUTED, LINE};
use crate::core::theme::metrics::{FACTS_GAP, FACTS_PAD_X, FACTS_PAD_Y, TEXT_STATUS};

const HALF: f32 = 0.5;

#[derive(IntoElement)]
pub struct QuickLookFacts {
    pub facts: Vec<(SharedString, SharedString)>,
}

impl RenderOnce for QuickLookFacts {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let cells = self.facts.into_iter().map(|(label, value)| {
            div()
                .w(relative(HALF))
                .flex()
                .flex_col()
                .child(div().text_color(rgb(FG_MUTED)).child(label))
                .child(
                    div()
                        .text_color(rgb(FG_BRIGHT))
                        .overflow_hidden()
                        .text_ellipsis()
                        .whitespace_nowrap()
                        .child(value),
                )
        });
        div()
            .flex()
            .flex_wrap()
            .flex_shrink_0()
            .gap_y(px(FACTS_GAP))
            .px(px(FACTS_PAD_X))
            .py(px(FACTS_PAD_Y))
            .border_t_1()
            .border_color(rgb(LINE))
            .text_size(px(TEXT_STATUS))
            .children(cells)
    }
}
