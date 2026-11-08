use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Div, IntoElement, MouseButton, Pixels, Size,
    Stateful, Window, anchored, deferred, div, point, prelude::*, px, rgba,
};

use crate::core::constants::preview::ZOOM_STEP;
use crate::core::theme::colors::SCRIM;
use crate::core::theme::metrics::{
    QUICK_LOOK_HEIGHT, QUICK_LOOK_MARGIN, QUICK_LOOK_SCROLL_STEP, QUICK_LOOK_WIDTH,
};
use crate::features::browser::presentation::actions::{
    CloseQuickLook, QuickLookNext, QuickLookPrevious, QuickLookScrollDown, QuickLookScrollUp,
    ZoomIn, ZoomOut,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::entry_icon::EntryIcon;
use crate::features::browser::presentation::components::quick_look_facts::QuickLookFacts;
use crate::features::browser::presentation::components::quick_look_header::QuickLookHeader;
use crate::features::browser::presentation::components::quick_look_panel::QuickLookPanel;
use crate::features::browser::presentation::key_bindings::QUICK_LOOK_CONTEXT;
use crate::features::browser::presentation::state::preview_state::PreviewState;
use crate::features::browser::presentation::state::preview_surface::PreviewSurface;
use crate::features::preview::domain::entities::preview_content::PreviewContent;
use crate::global_components::handlers::ClickHandler;

const OVERLAY_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;

impl BrowserView {
    fn turn_pdf_page(&mut self, delta: isize, cx: &mut Context<Self>) -> bool {
        let Some(state) = self.quick_look.as_ref() else {
            return false;
        };
        let Some(PreviewContent::Pdf { index, pages, .. }) = &state.content else {
            return false;
        };
        let last = pages.unwrap_or(*index) as isize;
        let target = (*index as isize + delta).clamp(1, last.max(1)) as usize;
        if target == *index {
            return true;
        }
        let (source, path) = (self.setup.preview.clone(), state.entry.path.clone());
        let shown = path.clone();
        self.quick_look_task = Some(cx.spawn(async move |this, cx| {
            let content = cx
                .background_executor()
                .spawn(async move { source.pdf_page(&path, target) })
                .await;
            let _ = this.update(cx, |this, cx| {
                if let Some(state) = this
                    .quick_look
                    .as_mut()
                    .filter(|state| state.entry.path == shown)
                {
                    state.content = Some(content);
                    cx.notify();
                }
            });
        }));
        true
    }

    fn scroll_quick_look(&mut self, direction: f32, cx: &mut Context<Self>) {
        if self.turn_pdf_page(direction.signum() as isize, cx) {
            return;
        }
        let handle = self.preview_scroll.0.borrow().base_handle.clone();
        let (offset, max) = (handle.offset(), handle.max_offset());
        let target = offset.y - px(QUICK_LOOK_SCROLL_STEP * direction);
        handle.set_offset(point(offset.x, target.clamp(-max.y, px(0.0))));
        cx.notify();
    }

    pub(super) fn quick_look_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let state = self.quick_look.clone()?;
        let viewport = window.viewport_size();
        let panel = self.quick_look_panel(&state, viewport, cx);
        let scrim = div()
            .id("quick-look")
            .track_focus(&self.quick_look_focus)
            .key_context(QUICK_LOOK_CONTEXT)
            .occlude()
            .w(viewport.width)
            .h(viewport.height)
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(SCRIM))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.close_quick_look(window, cx)),
            )
            .child(panel);
        let scrim = Self::with_quick_look_actions(scrim, cx);
        let overlay = anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.0), px(0.0)))
            .child(scrim);
        Some(
            deferred(overlay)
                .with_priority(OVERLAY_PRIORITY)
                .into_any_element(),
        )
    }

    fn quick_look_panel(
        &self,
        state: &PreviewState,
        viewport: Size<Pixels>,
        cx: &mut Context<Self>,
    ) -> QuickLookPanel {
        let margin = px(QUICK_LOOK_MARGIN * DOUBLE);
        let size = (
            px(QUICK_LOOK_WIDTH).min(viewport.width - margin),
            px(QUICK_LOOK_HEIGHT).min(viewport.height - margin),
        );
        let view = cx.entity().downgrade();
        let on_close: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_quick_look(window, cx));
        });
        let header = QuickLookHeader {
            icon: EntryIcon {
                entry: state.entry.clone(),
            }
            .into_any_element(),
            title: state.entry.name.clone().into(),
            on_close,
        };
        QuickLookPanel {
            header,
            stage: self.preview_stage(state, PreviewSurface::QuickLook, cx),
            facts: QuickLookFacts {
                facts: Self::preview_facts(state),
            },
            size,
        }
    }

    fn with_quick_look_actions(element: Stateful<Div>, cx: &mut Context<Self>) -> Stateful<Div> {
        element
            .on_action(
                cx.listener(|this, _: &CloseQuickLook, window, cx| {
                    this.close_quick_look(window, cx)
                }),
            )
            .on_action(cx.listener(|this, _: &QuickLookNext, _, cx| this.step_quick_look(1, cx)))
            .on_action(
                cx.listener(|this, _: &QuickLookScrollDown, _, cx| this.scroll_quick_look(1.0, cx)),
            )
            .on_action(
                cx.listener(|this, _: &QuickLookScrollUp, _, cx| this.scroll_quick_look(-1.0, cx)),
            )
            .on_action(
                cx.listener(|this, _: &QuickLookPrevious, _, cx| this.step_quick_look(-1, cx)),
            )
            .on_action(cx.listener(|this, _: &ZoomIn, _, cx| this.zoom_quick_look(ZOOM_STEP, cx)))
            .on_action(
                cx.listener(|this, _: &ZoomOut, _, cx| this.zoom_quick_look(1.0 / ZOOM_STEP, cx)),
            )
    }
}
