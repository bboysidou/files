use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{FG_SOFT, KBD_BORDER};
use crate::core::theme::metrics::{KBD_PAD_X, KBD_PAD_Y, KBD_RADIUS, TEXT_TINY};

#[derive(IntoElement)]
pub struct KbdHint {
    text: SharedString,
}

impl KbdHint {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for KbdHint {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .px(px(KBD_PAD_X))
            .py(px(KBD_PAD_Y))
            .rounded(px(KBD_RADIUS))
            .border_1()
            .border_color(rgb(KBD_BORDER))
            .text_size(px(TEXT_TINY))
            .text_color(rgb(FG_SOFT))
            .whitespace_nowrap()
            .child(self.text)
    }
}
