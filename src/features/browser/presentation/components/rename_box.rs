use gpui::{App, Entity, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{ACCENT, FG_BRIGHT, FRAME};
use crate::core::theme::metrics::{RENAME_MIN_WIDTH, RENAME_PAD_X, RENAME_PAD_Y, RENAME_RADIUS};
use crate::global_components::text_field::text_field_view::TextField;

#[derive(IntoElement)]
pub struct RenameBox {
    pub field: Entity<TextField>,
}

impl RenderOnce for RenameBox {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_1()
            .min_w(px(RENAME_MIN_WIDTH))
            .px(px(RENAME_PAD_X))
            .py(px(RENAME_PAD_Y))
            .rounded(px(RENAME_RADIUS))
            .bg(rgb(FRAME))
            .border_1()
            .border_color(rgb(ACCENT))
            .text_color(rgb(FG_BRIGHT))
            .child(self.field)
    }
}
