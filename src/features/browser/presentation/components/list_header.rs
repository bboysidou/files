use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb, rgba};

use crate::core::enums::browser::sort_key::SortKey;
use crate::core::theme::colors::{FG_MUTED, TRANSPARENT};
use crate::core::theme::metrics::{
    LIST_HEADER_PAD_BOTTOM, LIST_HEADER_PAD_TOP, LIST_KIND_WIDTH, LIST_MODIFIED_WIDTH,
    LIST_SIZE_WIDTH, TEXT_TINY,
};
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::presentation::components::header_cell::HeaderCell;
use crate::features::browser::presentation::state::handlers::SortHandler;

const COLUMNS: [(SortKey, Option<f32>, bool); 4] = [
    (SortKey::Name, None, false),
    (SortKey::Size, Some(LIST_SIZE_WIDTH), true),
    (SortKey::Kind, Some(LIST_KIND_WIDTH), false),
    (SortKey::Modified, Some(LIST_MODIFIED_WIDTH), false),
];

#[derive(IntoElement)]
pub struct ListHeader {
    pub sort: SortOrder,
    pub on_sort: SortHandler,
}

impl RenderOnce for ListHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let cells = COLUMNS.iter().map(|(key, width, align_end)| HeaderCell {
            key: *key,
            sort: self.sort,
            width: *width,
            align_end: *align_end,
            on_sort: self.on_sort.clone(),
        });
        div()
            .flex()
            .flex_shrink_0()
            .border_1()
            .border_color(rgba(TRANSPARENT))
            .pt(px(LIST_HEADER_PAD_TOP))
            .pb(px(LIST_HEADER_PAD_BOTTOM))
            .text_size(px(TEXT_TINY))
            .text_color(rgb(FG_MUTED))
            .children(cells)
    }
}
