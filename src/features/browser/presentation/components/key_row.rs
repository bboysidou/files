use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{FG, KEY_ROW_LINE};
use crate::core::theme::metrics::{KEY_COLUMN_GAP, KEY_ROW_PAD_Y, TEXT_SMALL};
use crate::global_components::kbd_hint::KbdHint;

#[derive(IntoElement)]
pub struct KeyRow {
    pub label: SharedString,
    pub keys: SharedString,
}

impl RenderOnce for KeyRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_between()
            .gap(px(KEY_COLUMN_GAP))
            .py(px(KEY_ROW_PAD_Y))
            .border_b_1()
            .border_color(rgb(KEY_ROW_LINE))
            .text_size(px(TEXT_SMALL))
            .text_color(rgb(FG))
            .child(self.label)
            .child(KbdHint::new(self.keys))
    }
}
