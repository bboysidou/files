use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Div, IntoElement, MouseButton, Pixels, Size,
    Stateful, Window, anchored, deferred, div, point, prelude::*, px, rgba,
};

use crate::core::constants::preview::ZOOM_STEP;
use crate::core::theme::colors::SCRIM;
use crate::core::theme::metrics::{QUICK_LOOK_HEIGHT, QUICK_LOOK_MARGIN, QUICK_LOOK_WIDTH};
use crate::features::browser::presentation::actions::{
    CloseQuickLook, QuickLookNext, QuickLookPrevious, ZoomIn, ZoomOut,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::entry_icon::EntryIcon;
use crate::features::browser::presentation::components::quick_look_facts::QuickLookFacts;
use crate::features::browser::presentation::components::quick_look_header::QuickLookHeader;
use crate::features::browser::presentation::components::quick_look_panel::QuickLookPanel;
use crate::features::browser::presentation::key_bindings::QUICK_LOOK_CONTEXT;
use crate::features::browser::presentation::state::preview_state::PreviewState;
use crate::features::browser::presentation::state::preview_surface::PreviewSurface;
use crate::global_components::handlers::ClickHandler;

const OVERLAY_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;

impl BrowserView {
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
                cx.listener(|this, _: &QuickLookPrevious, _, cx| this.step_quick_look(-1, cx)),
            )
            .on_action(cx.listener(|this, _: &ZoomIn, _, cx| this.zoom_quick_look(ZOOM_STEP, cx)))
            .on_action(
                cx.listener(|this, _: &ZoomOut, _, cx| this.zoom_quick_look(1.0 / ZOOM_STEP, cx)),
            )
    }
}
