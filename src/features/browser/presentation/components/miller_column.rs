use std::path::PathBuf;

use gpui::{App, ElementId, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb, rgba};

use crate::core::theme::colors::{ACCENT, DIVIDER, FG_SOFT, ROW_HOVER, SELECTION_ALPHA};
use crate::core::theme::metrics::{
    LIST_ROW_HEIGHT, LIST_ROW_RADIUS, LIST_ROW_SPACING, MILLER_COLUMN_WIDTH, TEXT_SMALL,
};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::components::name_cell::NameCell;
use crate::global_components::handlers::IndexHandler;

#[derive(IntoElement)]
pub struct MillerColumn {
    pub id: &'static str,
    pub entries: Vec<FileEntry>,
    pub highlight: Option<PathBuf>,
    pub on_select: IndexHandler,
}

impl RenderOnce for MillerColumn {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (highlight, on_select) = (self.highlight, self.on_select);
        let rows = self.entries.into_iter().enumerate().map(|(index, entry)| {
            let on_select = on_select.clone();
            let active = highlight.as_ref() == Some(&entry.path);
            let row = div()
                .id(ElementId::NamedInteger(self.id.into(), index as u64))
                .flex()
                .items_center()
                .h(px(LIST_ROW_HEIGHT))
                .rounded(px(LIST_ROW_RADIUS))
                .cursor_pointer()
                .on_click(move |_, window, cx| on_select(index, window, cx))
                .child(NameCell {
                    entry,
                    cut: false,
                    rename: None,
                    context: None,
                });
            let row = if active {
                row.bg(rgba((ACCENT << 8) | SELECTION_ALPHA))
            } else {
                row.hover(|style| style.bg(rgb(ROW_HOVER)))
            };
            div().py(px(LIST_ROW_SPACING)).child(row)
        });
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .flex_shrink_0()
            .w(px(MILLER_COLUMN_WIDTH))
            .h_full()
            .overflow_y_scroll()
            .border_r_1()
            .border_color(rgb(DIVIDER))
            .text_size(px(TEXT_SMALL))
            .text_color(rgb(FG_SOFT))
            .children(rows)
    }
}
