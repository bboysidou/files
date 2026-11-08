use std::ops::Range;
use std::rc::Rc;
use std::time::SystemTime;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Div, IntoElement, MouseButton, Pixels, SharedString,
    Size, Stateful, Window, anchored, deferred, div, point, prelude::*, px, rgba, uniform_list,
};
use rust_i18n::t;

use crate::core::constants::files::MISSING_VALUE;
use crate::core::constants::preview::ZOOM_STEP;
use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::format::size_text::size_text;
use crate::core::helpers::time::relative_time::relative_time;
use crate::core::theme::colors::SCRIM;
use crate::core::theme::metrics::{
    QUICK_LOOK_HEIGHT, QUICK_LOOK_MARGIN, QUICK_LOOK_STAGE_PAD, QUICK_LOOK_WIDTH,
};
use crate::features::browser::presentation::actions::{
    CloseQuickLook, QuickLookNext, QuickLookPrevious, ZoomIn, ZoomOut,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::entry_icon::EntryIcon;
use crate::features::browser::presentation::components::preview_folder::PreviewFolder;
use crate::features::browser::presentation::components::preview_image::PreviewImage;
use crate::features::browser::presentation::components::preview_line::PreviewLine;
use crate::features::browser::presentation::components::quick_look_facts::QuickLookFacts;
use crate::features::browser::presentation::components::quick_look_header::QuickLookHeader;
use crate::features::browser::presentation::components::quick_look_panel::QuickLookPanel;
use crate::features::browser::presentation::key_bindings::QUICK_LOOK_CONTEXT;
use crate::features::browser::presentation::state::quick_look_state::QuickLookState;
use crate::features::preview::domain::entities::preview_content::PreviewContent;
use crate::global_components::empty_state::EmptyState;
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
        state: &QuickLookState,
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
            stage: self.stage(state, cx),
            facts: QuickLookFacts {
                facts: Self::facts(state),
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

    fn stage(&self, state: &QuickLookState, cx: &mut Context<Self>) -> AnyElement {
        let empty = |icon: Icon, title: String, body: String| {
            EmptyState {
                icon,
                title: title.into(),
                body: body.into(),
            }
            .into_any_element()
        };
        match &state.content {
            None => div().into_any_element(),
            Some(PreviewContent::Image { path, .. }) => PreviewImage {
                path: path.clone(),
                zoom: state.zoom,
            }
            .into_any_element(),
            Some(PreviewContent::Text { lines, .. }) => uniform_list(
                "preview-lines",
                lines.len(),
                cx.processor(|this, range: Range<usize>, _window, _cx| this.preview_lines(range)),
            )
            .size_full()
            .p(px(QUICK_LOOK_STAGE_PAD))
            .track_scroll(&self.preview_scroll)
            .into_any_element(),
            Some(PreviewContent::Folder { entries, total }) => div()
                .size_full()
                .p(px(QUICK_LOOK_STAGE_PAD))
                .child(PreviewFolder {
                    entries: entries.clone(),
                    total: *total,
                })
                .into_any_element(),
            Some(PreviewContent::Empty) => empty(
                Icon::Document,
                t!("ql.empty_file").to_string(),
                t!("ql.empty_file.body").to_string(),
            ),
            Some(PreviewContent::Unsupported) => empty(
                Icon::Document,
                t!(state.entry.kind.label_key()).to_string(),
                t!("ql.unsupported.body").to_string(),
            ),
            Some(PreviewContent::Failed) => empty(
                Icon::Document,
                t!("ql.failed.title").to_string(),
                t!("ql.failed.body").to_string(),
            ),
        }
    }

    fn preview_lines(&mut self, range: Range<usize>) -> Vec<PreviewLine> {
        let Some(Some(PreviewContent::Text { lines, .. })) =
            self.quick_look.as_ref().map(|state| &state.content)
        else {
            return Vec::new();
        };
        range
            .filter_map(|index| {
                Some(PreviewLine {
                    number: index + 1,
                    text: lines.get(index)?.clone().into(),
                })
            })
            .collect()
    }

    fn facts(state: &QuickLookState) -> Vec<(SharedString, SharedString)> {
        let entry = &state.entry;
        let modified = entry.modified.map_or_else(
            || MISSING_VALUE.to_string(),
            |moment| relative_time(moment, SystemTime::now()),
        );
        let mut facts = vec![
            (
                t!("ql.fact.size").to_string(),
                size_text(entry.is_dir, entry.child_count, entry.size),
            ),
            (t!("ql.fact.modified").to_string(), modified),
            (
                t!("ql.fact.kind").to_string(),
                t!(entry.kind.label_key()).to_string(),
            ),
        ];
        facts.extend(state.content.as_ref().and_then(Self::content_fact));
        facts
            .into_iter()
            .map(|(label, value)| (label.into(), value.into()))
            .collect()
    }

    fn content_fact(content: &PreviewContent) -> Option<(String, String)> {
        match content {
            PreviewContent::Image {
                dimensions: Some((width, height)),
                ..
            } => Some((
                t!("ql.fact.dimensions").to_string(),
                format!("{width} × {height}"),
            )),
            PreviewContent::Text {
                lines,
                truncated: true,
            } => Some((
                t!("ql.fact.lines").to_string(),
                t!("ql.lines_truncated", count = lines.len()).to_string(),
            )),
            PreviewContent::Text {
                lines,
                truncated: false,
            } => Some((t!("ql.fact.lines").to_string(), lines.len().to_string())),
            _ => None,
        }
    }
}
