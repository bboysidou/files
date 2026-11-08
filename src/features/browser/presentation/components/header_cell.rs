use gpui::{
    App, AppContext, EmptyView, IntoElement, MouseButton, RenderOnce, Window, div, prelude::*, px,
    rgb,
};
use rust_i18n::t;

use crate::core::enums::browser::sort_key::SortKey;
use crate::core::theme::colors::{ACCENT, FG_BRIGHT};
use crate::core::theme::metrics::{COLUMN_GRIP_WIDTH, LIST_CELL_PAD_X, SORT_ARROW_GAP};
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::presentation::state::column_drag::ColumnDrag;
use crate::features::browser::presentation::state::handlers::SortHandler;

const ARROW_UP: &str = "↑";
const ARROW_DOWN: &str = "↓";

#[derive(IntoElement)]
pub struct HeaderCell {
    pub key: SortKey,
    pub sort: SortOrder,
    pub width: Option<f32>,
    pub align_end: bool,
    pub on_sort: SortHandler,
}

impl RenderOnce for HeaderCell {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (key, on_sort) = (self.key, self.on_sort);
        let cell = div()
            .id(("sort", key as usize))
            .flex()
            .items_center()
            .gap(px(SORT_ARROW_GAP))
            .px(px(LIST_CELL_PAD_X))
            .cursor_pointer()
            .whitespace_nowrap()
            .hover(|style| style.text_color(rgb(FG_BRIGHT)))
            .on_click(move |_, window, cx| on_sort(key, window, cx))
            .child(t!(key.label_key()).to_string());
        let cell = match self.width {
            Some(width) => cell.relative().min_w_0().w(px(width)).child(
                div()
                    .id(("grip", key as usize))
                    .absolute()
                    .left_0()
                    .top_0()
                    .bottom_0()
                    .w(px(COLUMN_GRIP_WIDTH))
                    .cursor_col_resize()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_drag(ColumnDrag { key }, |_, _, _, cx| cx.new(|_| EmptyView)),
            ),
            None => cell.flex_1().min_w_0(),
        };
        let cell = if self.align_end {
            cell.justify_end()
        } else {
            cell
        };
        if self.sort.key != key {
            return cell;
        }
        let arrow = if self.sort.ascending {
            ARROW_UP
        } else {
            ARROW_DOWN
        };
        cell.child(div().text_color(rgb(ACCENT)).child(arrow))
    }
}
