use std::time::SystemTime;

use gpui::{
    App, Entity, IntoElement, MouseButton, RenderOnce, SharedString, Window, div, prelude::*, px,
    rgb, rgba,
};
use rust_i18n::t;

use crate::core::constants::files::MISSING_VALUE;
use crate::core::enums::browser::sort_key::SortKey;
use crate::core::helpers::format::size_text::size_text;
use crate::core::helpers::time::relative_time::relative_time;
use crate::core::theme::colors::{
    ACCENT, FG_MUTED, FG_SOFT, ROW_HOVER, SELECTION_ALPHA, TRANSPARENT,
};
use crate::core::theme::metrics::TREE_INDENT;
use crate::core::theme::metrics::{
    LIST_CELL_PAD_X, LIST_ROW_HEIGHT, LIST_ROW_RADIUS, LIST_ROW_SPACING, TEXT_SMALL,
};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::components::name_cell::NameCell;
use crate::global_components::drag::drag_slot::DragSlot;
use crate::global_components::handlers::{IndexClickHandler, IndexPointHandler};
use crate::global_components::text_field::text_field_view::TextField;

#[derive(IntoElement)]
pub struct FileRow {
    pub index: usize,
    pub depth: usize,
    pub drag: DragSlot,
    pub entry: FileEntry,
    pub marked: bool,
    pub cursor: bool,
    pub focused: bool,
    pub cut: bool,
    pub rename: Option<Entity<TextField>>,
    pub now: SystemTime,
    pub on_click: IndexClickHandler,
    pub on_right: IndexPointHandler,
    pub columns: Vec<(SortKey, u16)>,
    pub name_only: bool,
    pub context: Option<String>,
}

impl FileRow {
    fn size_text(entry: &FileEntry) -> SharedString {
        size_text(entry.is_dir, entry.child_count, entry.size).into()
    }

    fn column_text(key: SortKey, entry: &FileEntry, now: SystemTime) -> String {
        let when = |moment: Option<SystemTime>| {
            moment.map_or_else(
                || MISSING_VALUE.to_string(),
                |moment| relative_time(moment, now),
            )
        };
        match key {
            SortKey::Size => Self::size_text(entry).to_string(),
            SortKey::Kind => t!(entry.kind.label_key()).to_string(),
            SortKey::Created => when(entry.created),
            _ => when(entry.modified),
        }
    }

    fn cells(&self) -> Vec<gpui::Div> {
        self.columns
            .iter()
            .map(|(key, width)| {
                let cell = Self::cell(
                    f32::from(*width),
                    Self::column_text(*key, &self.entry, self.now),
                );
                if *key == SortKey::Size {
                    cell.flex().justify_end()
                } else {
                    cell
                }
            })
            .collect()
    }

    fn cell(width: f32, text: impl Into<SharedString>) -> gpui::Div {
        div()
            .min_w_0()
            .w(px(width))
            .px(px(LIST_CELL_PAD_X))
            .overflow_hidden()
            .whitespace_nowrap()
            .text_ellipsis()
            .child(text.into())
    }
}

impl RenderOnce for FileRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let cells = self.cells();
        let (index, on_click, on_right) = (self.index, self.on_click, self.on_right);
        let row = div()
            .id(("row", index))
            .flex()
            .items_center()
            .h(px(LIST_ROW_HEIGHT))
            .rounded(px(LIST_ROW_RADIUS))
            .border_1()
            .border_color(match (self.cursor, self.focused) {
                (true, true) => rgb(ACCENT),
                (true, false) => rgb(FG_MUTED),
                (false, _) => rgba(TRANSPARENT),
            })
            .text_size(px(TEXT_SMALL))
            .text_color(rgb(FG_SOFT))
            .on_click(move |event, window, cx| on_click(index, event, window, cx))
            .on_mouse_down(MouseButton::Right, move |event, window, cx| {
                cx.stop_propagation();
                on_right(index, event.position, window, cx)
            });
        let row = self.drag.apply(row);
        let row = if self.marked {
            row.bg(rgba((ACCENT << 8) | SELECTION_ALPHA))
        } else {
            row.hover(|style| style.bg(rgb(ROW_HOVER)))
        };
        div().py(px(LIST_ROW_SPACING)).child(
            row.child(
                div()
                    .flex()
                    .flex_1()
                    .min_w_0()
                    .pl(px(TREE_INDENT * self.depth as f32))
                    .child(NameCell {
                        entry: self.entry,
                        cut: self.cut,
                        rename: self.rename,
                        context: self.context,
                    }),
            )
            .when(!self.name_only, |row| row.children(cells)),
        )
    }
}
