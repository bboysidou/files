use gpui::{App, DragMoveEvent, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb, rgba};

use crate::core::enums::browser::sort_key::SortKey;
use crate::core::theme::colors::{FG_MUTED, TRANSPARENT};
use crate::core::theme::metrics::{LIST_HEADER_PAD_BOTTOM, LIST_HEADER_PAD_TOP, TEXT_TINY};
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::presentation::components::header_cell::HeaderCell;
use crate::features::browser::presentation::state::column_drag::ColumnDrag;
use crate::features::browser::presentation::state::handlers::{ResizeHandler, SortHandler};

const BORDER: f32 = 1.0;

#[derive(IntoElement)]
pub struct ListHeader {
    pub sort: SortOrder,
    pub on_sort: SortHandler,
    pub columns: Vec<(SortKey, u16)>,
    pub on_resize: ResizeHandler,
}

impl RenderOnce for ListHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let name = HeaderCell {
            key: SortKey::Name,
            sort: self.sort,
            width: None,
            align_end: false,
            on_sort: self.on_sort.clone(),
        };
        let cells = self.columns.iter().map(|(key, width)| HeaderCell {
            key: *key,
            sort: self.sort,
            width: Some(f32::from(*width)),
            align_end: *key == SortKey::Size,
            on_sort: self.on_sort.clone(),
        });
        let (columns, on_resize) = (self.columns.clone(), self.on_resize);
        div()
            .id("list-header")
            .flex()
            .flex_shrink_0()
            .border_1()
            .border_color(rgba(TRANSPARENT))
            .pt(px(LIST_HEADER_PAD_TOP))
            .pb(px(LIST_HEADER_PAD_BOTTOM))
            .text_size(px(TEXT_TINY))
            .text_color(rgb(FG_MUTED))
            .on_drag_move(move |event: &DragMoveEvent<ColumnDrag>, window, cx| {
                let key = event.drag(cx).key;
                let Some(position) = columns.iter().position(|(column, _)| *column == key) else {
                    return;
                };
                let after: f32 = columns[position + 1..]
                    .iter()
                    .map(|(_, width)| f32::from(*width))
                    .sum();
                let right = event.bounds.right() / px(1.0) - BORDER - after;
                let width = (right - event.event.position.x / px(1.0)).max(0.0) as u16;
                on_resize(key, width, window, cx);
            })
            .child(name)
            .children(cells)
    }
}
