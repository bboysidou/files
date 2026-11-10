use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, relative, rgb, rgba};

use crate::core::enums::compare::diff_kind::DiffKind;
use crate::core::theme::colors::{
    DIFF_ADDED, DIFF_CHANGED, DIFF_REMOVED, FG, FG_DIM, LINE, TRANSPARENT,
};
use crate::core::theme::metrics::{
    DIFF_GUTTER, DIFF_NUMBER_WIDTH, PREVIEW_LINE_HEIGHT, TEXT_SMALL,
};
use crate::features::compare::domain::entities::diff_row::DiffRow;

const HALF: f32 = 0.5;

#[derive(IntoElement)]
pub struct DiffLine {
    pub row: DiffRow,
}

fn tint(kind: DiffKind, left_side: bool) -> u32 {
    match (kind, left_side) {
        (DiffKind::Changed, _) => DIFF_CHANGED,
        (DiffKind::Removed, true) => DIFF_REMOVED,
        (DiffKind::Added, false) => DIFF_ADDED,
        _ => TRANSPARENT,
    }
}

fn side(side: Option<(usize, String)>, color: u32) -> impl IntoElement {
    let (number, text) = side.map_or((String::new(), String::new()), |(number, text)| {
        (number.to_string(), text)
    });
    div()
        .w(relative(HALF))
        .h_full()
        .flex()
        .overflow_hidden()
        .bg(if color == TRANSPARENT {
            rgba(TRANSPARENT)
        } else {
            rgb(color)
        })
        .child(
            div()
                .w(px(DIFF_NUMBER_WIDTH))
                .flex_shrink_0()
                .text_color(rgb(FG_DIM))
                .child(number),
        )
        .child(div().whitespace_nowrap().text_color(rgb(FG)).child(text))
}

impl RenderOnce for DiffLine {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let kind = self.row.kind;
        div()
            .w_full()
            .h(px(PREVIEW_LINE_HEIGHT))
            .flex()
            .text_size(px(TEXT_SMALL))
            .child(side(self.row.left, tint(kind, true)))
            .child(div().w(px(DIFF_GUTTER)).h_full().bg(rgb(LINE)))
            .child(side(self.row.right, tint(kind, false)))
    }
}
