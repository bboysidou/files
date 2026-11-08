use std::ops::Range;

use gpui::{
    AnyElement, Bounds, Context, Div, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent,
    Pixels, Point, div, point, prelude::*, px, rgb, rgba,
};

use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::colors::{ACCENT, BAND_ALPHA};
use crate::core::theme::metrics::TILE_GAP;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::band_state::BandState;

impl PaneView {
    pub(super) fn band_area(&self, list: impl IntoElement, cx: &mut Context<Self>) -> Div {
        div()
            .relative()
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .size_full()
            .on_mouse_down(MouseButton::Left, cx.listener(Self::start_band))
            .on_mouse_move(cx.listener(Self::drag_band))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.end_band(cx)),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.end_band(cx)),
            )
            .child(list)
            .children(self.band_overlay())
    }

    pub(super) fn viewport(&self) -> (Bounds<Pixels>, Point<Pixels>, Pixels) {
        let state = self.scroll.0.borrow();
        let pitch = state
            .last_item_size
            .map_or(px(1.0), |size| size.item.height.max(px(1.0)));
        (
            state.base_handle.bounds(),
            state.base_handle.offset(),
            pitch,
        )
    }

    fn content_point(&self, position: Point<Pixels>) -> Point<Pixels> {
        let (bounds, offset, _) = self.viewport();
        point(
            position.x - bounds.origin.x,
            position.y - bounds.origin.y - offset.y,
        )
    }

    fn start_band(&mut self, event: &MouseDownEvent, _: &mut gpui::Window, cx: &mut Context<Self>) {
        let origin = self.content_point(event.position);
        if self.band_hits(origin, origin).next().is_some() {
            return;
        }
        let keep = event.modifiers.control || event.modifiers.shift;
        let base = if keep {
            self.selection.marked().collect()
        } else {
            Vec::new()
        };
        self.selection.set_marked(base.iter().copied());
        self.band = Some(BandState {
            origin,
            current: origin,
            pointer: event.position,
            base,
            ticker: None,
        });
        cx.notify();
    }

    fn drag_band(&mut self, event: &MouseMoveEvent, _: &mut gpui::Window, cx: &mut Context<Self>) {
        if self.band.is_none() || !event.dragging() {
            return;
        }
        if let Some(band) = self.band.as_mut() {
            band.pointer = event.position;
        }
        self.update_band(cx);
        self.follow_band_edge(cx);
    }

    pub(super) fn update_band(&mut self, cx: &mut Context<Self>) {
        let Some(pointer) = self.band.as_ref().map(|band| band.pointer) else {
            return;
        };
        let current = self.content_point(pointer);
        let Some(band) = self.band.as_mut() else {
            return;
        };
        band.current = current;
        let origin = band.origin;
        let mut marked = band.base.clone();
        marked.extend(self.band_hits(origin, current));
        self.selection.set_marked(marked);
        cx.notify();
    }

    fn end_band(&mut self, cx: &mut Context<Self>) {
        if self.band.take().is_some() {
            cx.notify();
        }
    }

    fn band_hits(&self, a: Point<Pixels>, b: Point<Pixels>) -> impl Iterator<Item = usize> + '_ {
        let (_, _, pitch) = self.viewport();
        let (top, bottom) = (a.y.min(b.y) / pitch, a.y.max(b.y) / pitch);
        let (left, right) = (a.x.min(b.x) / px(1.0), a.x.max(b.x) / px(1.0));
        let rows = top.max(0.0).floor() as usize..bottom.max(0.0).floor() as usize + 1;
        let grid = self.view == ViewMode::Grid;
        let layout = self.layout();
        let columns = if grid { layout.columns } else { 1 };
        let gap_share = if grid {
            TILE_GAP / (pitch / px(1.0))
        } else {
            0.0
        };
        let in_gap = top.fract() > 1.0 - gap_share && top.floor() == bottom.floor();
        let cells = if grid {
            column_span(left, right, layout.tile_width)
        } else {
            0..1
        };
        rows.filter(move |_| !in_gap)
            .flat_map(move |row| cells.clone().map(move |column| row * columns + column))
            .filter(move |index| *index < self.entries.len())
    }

    fn band_overlay(&self) -> Option<AnyElement> {
        let band = self.band.as_ref()?;
        let (_, offset, _) = self.viewport();
        let (left, top) = (
            band.origin.x.min(band.current.x),
            band.origin.y.min(band.current.y),
        );
        let width = (band.origin.x - band.current.x).abs();
        let height = (band.origin.y - band.current.y).abs();
        Some(
            div()
                .absolute()
                .left(left)
                .top(top + offset.y)
                .w(width)
                .h(height)
                .border_1()
                .border_color(rgb(ACCENT))
                .bg(rgba((ACCENT << 8) | BAND_ALPHA))
                .into_any_element(),
        )
    }
}

fn column_span(left: f32, right: f32, tile_width: f32) -> Range<usize> {
    let step = tile_width + TILE_GAP;
    let first = (left.max(0.0) / step).floor();
    let first = if left.max(0.0) - first * step > tile_width {
        first + 1.0
    } else {
        first
    };
    let last = (right.max(0.0) / step).floor() as usize;
    first as usize..last + 1
}
