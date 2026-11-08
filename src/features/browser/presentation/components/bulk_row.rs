use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{DANGER, FG, FG_BRIGHT, FG_MUTED};
use crate::core::theme::metrics::{FOLDER_PREVIEW_GAP, FOLDER_PREVIEW_PAD_X, FOLDER_PREVIEW_PAD_Y};

const ARROW: &str = "→";

#[derive(IntoElement)]
pub struct BulkRow {
    pub old: SharedString,
    pub new: SharedString,
    pub conflict: bool,
}

impl RenderOnce for BulkRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let changed = self.old != self.new;
        let new_color = match (self.conflict, changed) {
            (true, _) => DANGER,
            (false, true) => FG_BRIGHT,
            (false, false) => FG_MUTED,
        };
        let cell = |text: SharedString, color: u32| {
            div()
                .flex_1()
                .min_w_0()
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .text_color(rgb(color))
                .child(text)
        };
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(FOLDER_PREVIEW_GAP))
            .px(px(FOLDER_PREVIEW_PAD_X))
            .py(px(FOLDER_PREVIEW_PAD_Y))
            .child(cell(self.old, FG))
            .child(div().flex_shrink_0().text_color(rgb(FG_MUTED)).child(ARROW))
            .child(cell(self.new, new_color))
    }
}
