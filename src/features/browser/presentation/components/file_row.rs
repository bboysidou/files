use std::time::SystemTime;

use gpui::{
    App, Entity, IntoElement, MouseButton, RenderOnce, SharedString, Window, div, prelude::*, px,
    rgb, rgba,
};
use rust_i18n::t;

use crate::core::constants::files::MISSING_VALUE;
use crate::core::helpers::format::size_text::size_text;
use crate::core::helpers::time::relative_time::relative_time;
use crate::core::theme::colors::{ACCENT, FG_SOFT, ROW_HOVER, SELECTION_ALPHA, TRANSPARENT};
use crate::core::theme::metrics::{
    LIST_CELL_PAD_X, LIST_KIND_WIDTH, LIST_MODIFIED_WIDTH, LIST_ROW_HEIGHT, LIST_ROW_RADIUS,
    LIST_ROW_SPACING, LIST_SIZE_WIDTH, TEXT_SMALL,
};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::components::name_cell::NameCell;
use crate::global_components::drag::drag_slot::DragSlot;
use crate::global_components::handlers::{IndexClickHandler, IndexPointHandler};
use crate::global_components::text_field::text_field_view::TextField;

#[derive(IntoElement)]
pub struct FileRow {
    pub index: usize,
    pub drag: DragSlot,
    pub entry: FileEntry,
    pub marked: bool,
    pub cursor: bool,
    pub cut: bool,
    pub rename: Option<Entity<TextField>>,
    pub now: SystemTime,
    pub on_click: IndexClickHandler,
    pub on_right: IndexPointHandler,
    pub compact: bool,
    pub name_only: bool,
    pub context: Option<String>,
}

impl FileRow {
    fn size_text(entry: &FileEntry) -> SharedString {
        size_text(entry.is_dir, entry.child_count, entry.size).into()
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
        let (index, on_click, on_right) = (self.index, self.on_click, self.on_right);
        let size = Self::size_text(&self.entry);
        let kind = t!(self.entry.kind.label_key()).to_string();
        let modified = self.entry.modified.map_or_else(
            || MISSING_VALUE.to_string(),
            |moment| relative_time(moment, self.now),
        );
        let row = div()
            .id(("row", index))
            .flex()
            .items_center()
            .h(px(LIST_ROW_HEIGHT))
            .rounded(px(LIST_ROW_RADIUS))
            .border_1()
            .border_color(if self.cursor {
                rgb(ACCENT)
            } else {
                rgba(TRANSPARENT)
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
            row.child(NameCell {
                entry: self.entry,
                cut: self.cut,
                rename: self.rename,
                context: self.context,
            })
            .when(!self.name_only, |row| {
                row.child(Self::cell(LIST_SIZE_WIDTH, size).flex().justify_end())
                    .when(!self.compact, |row| {
                        row.child(Self::cell(LIST_KIND_WIDTH, kind))
                    })
                    .child(Self::cell(LIST_MODIFIED_WIDTH, modified))
            }),
        )
    }
}
