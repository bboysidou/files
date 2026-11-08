use std::ops::Range;
use std::rc::Rc;

use gpui::{AnyElement, Context, Div, IntoElement, canvas, div, prelude::*, px, uniform_list};

use crate::core::enums::view::view_mode::ViewMode;
use crate::core::helpers::format::grid_layout::{GridLayout, grid_layout};
use crate::core::theme::metrics::TILE_GAP;
use crate::features::browser::presentation::components::file_tile::FileTile;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::row_click::RowClick;
use crate::global_components::handlers::IndexClickHandler;

const WIDTH_TOLERANCE: f32 = 0.5;

impl PaneView {
    pub(super) fn layout(&self) -> GridLayout {
        grid_layout(self.grid_width)
    }

    pub(super) fn row_step(&self) -> isize {
        match self.view {
            ViewMode::Grid => self.layout().columns as isize,
            _ => 1,
        }
    }

    pub(super) fn visual_row(&self, index: usize) -> usize {
        match self.view {
            ViewMode::Grid => index / self.layout().columns,
            _ => index,
        }
    }

    pub(super) fn grid(&self, cx: &mut Context<Self>) -> AnyElement {
        let rows = self.entries.len().div_ceil(self.layout().columns);
        let view = cx.entity().downgrade();
        let probe = canvas(
            move |bounds, _window, cx| {
                let width = bounds.size.width / px(1.0);
                let _ = view.update(cx, |this, cx| {
                    if (this.grid_width - width).abs() > WIDTH_TOLERANCE {
                        this.grid_width = width;
                        cx.notify();
                    }
                });
            },
            |_, _, _, _| {},
        )
        .absolute()
        .size_full();
        let list = uniform_list(
            "grid-rows",
            rows,
            cx.processor(|this, range: Range<usize>, _window, cx| this.grid_rows(range, cx)),
        )
        .size_full()
        .track_scroll(&self.scroll);
        div()
            .relative()
            .size_full()
            .child(probe)
            .child(list)
            .into_any_element()
    }

    fn grid_rows(&mut self, range: Range<usize>, cx: &mut Context<Self>) -> Vec<Div> {
        let layout = self.layout();
        let view = cx.entity().downgrade();
        let on_click: IndexClickHandler = Rc::new(move |index, event, window, cx| {
            let _ = view.update(cx, |this, cx| {
                this.click_row(RowClick::from_event(index, event), window, cx)
            });
        });
        let mut rows = Vec::new();
        for row in range {
            let start = row * layout.columns;
            let end = (start + layout.columns).min(self.entries.len());
            let mut tiles = Vec::new();
            for index in start..end {
                let Some(entry) = self.entries.get(index).cloned() else {
                    continue;
                };
                let thumbnail = self.thumbnail_for(&entry, cx);
                tiles.push(FileTile {
                    index,
                    marked: self.selection.is_marked(index),
                    cursor: index == self.selection.cursor(),
                    cut: self.cut.contains(&entry.path),
                    entry,
                    thumbnail,
                    width: layout.tile_width,
                    thumb_height: layout.thumb_height,
                    on_click: on_click.clone(),
                });
            }
            rows.push(
                div()
                    .flex()
                    .gap(px(TILE_GAP))
                    .h(px(layout.row_height()))
                    .children(tiles),
            );
        }
        rows
    }
}
