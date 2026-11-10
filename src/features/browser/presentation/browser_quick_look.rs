use std::rc::Rc;

use gpui::{Context, Focusable, Pixels, Point, Size, Window, px};

use crate::core::constants::preview::{PAN_HALF, ZOOM_DEFAULT, ZOOM_MAX, ZOOM_MIN};
use crate::core::theme::metrics::QUICK_LOOK_STAGE_PAD;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::pan_gestures::PanGestures;
use crate::features::browser::presentation::state::preview_state::PreviewState;
use crate::features::browser::presentation::state::preview_surface::PreviewSurface;
use crate::features::preview::domain::entities::preview_content::PreviewContent;

fn image_dimensions(state: &PreviewState) -> Option<(f32, f32)> {
    let (width, height) = match state.content.as_ref()? {
        PreviewContent::Image { dimensions, .. } | PreviewContent::Pdf { dimensions, .. } => {
            (*dimensions)?
        }
        _ => return None,
    };
    (width > 0 && height > 0).then_some((width as f32, height as f32))
}

fn pan_limits(state: &PreviewState, stage: Size<Pixels>) -> (f32, f32) {
    let inner_w = f32::from(stage.width) - QUICK_LOOK_STAGE_PAD * 2.0;
    let inner_h = f32::from(stage.height) - QUICK_LOOK_STAGE_PAD * 2.0;
    let box_limit = ((state.zoom - 1.0) * PAN_HALF).max(0.0);
    let Some((image_w, image_h)) =
        image_dimensions(state).filter(|_| inner_w > 0.0 && inner_h > 0.0)
    else {
        return (box_limit, box_limit);
    };
    let fit = (inner_w / image_w).min(inner_h / image_h) * state.zoom;
    let limit = |shown: f32, inner: f32| ((shown - inner) * PAN_HALF).max(0.0) / inner;
    (limit(image_w * fit, inner_w), limit(image_h * fit, inner_h))
}

fn clamp_pan(state: &mut PreviewState, stage: Size<Pixels>) {
    let (limit_x, limit_y) = pan_limits(state, stage);
    state.pan_x = state.pan_x.clamp(-limit_x, limit_x);
    state.pan_y = state.pan_y.clamp(-limit_y, limit_y);
}

impl BrowserView {
    pub(super) fn open_quick_look(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self.active_pane().read(cx).current_entry() else {
            return;
        };
        window.focus(&self.quick_look.focus, cx);
        self.show_preview(entry, cx);
    }

    pub(super) fn close_quick_look(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.quick_look.state = None;
        self.quick_look.task = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn step_quick_look(&mut self, delta: isize, cx: &mut Context<Self>) {
        let pane = self.active_pane().clone();
        pane.update(cx, |pane, cx| pane.move_cursor(delta, cx));
        let Some(entry) = pane.read(cx).current_entry() else {
            return;
        };
        let unchanged = self
            .quick_look
            .state
            .as_ref()
            .is_some_and(|state| state.entry.path == entry.path);
        if !unchanged {
            self.show_preview(entry, cx);
        }
    }

    pub(super) fn zoom_quick_look(&mut self, factor: f32, cx: &mut Context<Self>) {
        let stage = self.quick_look.stage.get();
        if let Some(state) = self.quick_look.state.as_mut() {
            state.zoom = (state.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
            clamp_pan(state, stage);
            cx.notify();
        }
    }

    pub(super) fn reset_quick_look_zoom(&mut self, cx: &mut Context<Self>) {
        let stage = self.quick_look.stage.get();
        if let Some(state) = self.quick_look.state.as_mut() {
            state.zoom = ZOOM_DEFAULT;
            clamp_pan(state, stage);
            cx.notify();
        }
    }

    pub(super) fn pan_quick_look(&mut self, (dx, dy): (f32, f32), cx: &mut Context<Self>) {
        let stage = self.quick_look.stage.get();
        if let Some(state) = self.quick_look.state.as_mut() {
            state.pan_x += dx;
            state.pan_y += dy;
            clamp_pan(state, stage);
            cx.notify();
        }
    }

    pub(super) fn drag_quick_look(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let stage = self.quick_look.stage.get();
        if let Some(last) = self.quick_look.pan_from.replace(position)
            && stage.width > px(0.0)
            && stage.height > px(0.0)
        {
            let dx = (last.x - position.x) / stage.width;
            let dy = (last.y - position.y) / stage.height;
            self.pan_quick_look((dx, dy), cx);
        }
    }

    fn show_preview(&mut self, entry: FileEntry, cx: &mut Context<Self>) {
        self.begin_preview(entry, PreviewSurface::QuickLook, cx);
    }

    pub(super) fn pan_gestures(&self, cx: &mut Context<Self>) -> PanGestures {
        let (press, drag, release) = (
            cx.entity().downgrade(),
            cx.entity().downgrade(),
            cx.entity().downgrade(),
        );
        PanGestures {
            stage: self.quick_look.stage.clone(),
            on_press: Rc::new(move |position, _, cx| {
                let _ = press.update(cx, |this, _| this.quick_look.pan_from = Some(position));
            }),
            on_drag: Rc::new(move |position, _, cx| {
                let _ = drag.update(cx, |this, cx| this.drag_quick_look(position, cx));
            }),
            on_release: Rc::new(move |_, cx| {
                let _ = release.update(cx, |this, _| this.quick_look.pan_from = None);
            }),
        }
    }
}
