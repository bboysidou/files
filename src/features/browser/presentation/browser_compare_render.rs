use std::ops::Range;
use std::path::Path;
use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Div, IntoElement, MouseButton, Stateful, Window,
    anchored, deferred, div, point, prelude::*, px, rgb, rgba, uniform_list,
};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::format::file_size::file_size;
use crate::core::theme::colors::{FG, FG_MUTED, SCRIM};
use crate::core::theme::metrics::{
    QUICK_LOOK_HEIGHT, QUICK_LOOK_MARGIN, QUICK_LOOK_STAGE_PAD, QUICK_LOOK_WIDTH,
};
use crate::features::browser::presentation::actions::{
    CloseCompare, CompareFiles, CompareScrollDown, CompareScrollUp,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::diff_line::DiffLine;
use crate::features::browser::presentation::components::preview_image::PreviewImage;
use crate::features::browser::presentation::components::quick_look_facts::QuickLookFacts;
use crate::features::browser::presentation::components::quick_look_header::QuickLookHeader;
use crate::features::browser::presentation::components::quick_look_panel::QuickLookPanel;
use crate::features::browser::presentation::key_bindings::COMPARE_CONTEXT;
use crate::features::compare::domain::entities::compare_result::CompareResult;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_glyph::IconGlyph;

const OVERLAY_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;
const HALF: f32 = 0.5;

type Facts = Vec<(gpui::SharedString, gpui::SharedString)>;

fn name_of(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn fact(label: &str, value: String) -> (gpui::SharedString, gpui::SharedString) {
    (t!(label).to_string().into(), value.into())
}

fn facts_of(result: Option<&CompareResult>) -> Facts {
    match result {
        None => vec![fact(
            "compare.fact.status",
            t!("compare.comparing").to_string(),
        )],
        Some(CompareResult::Text { changes, .. }) => vec![fact(
            "compare.fact.changes",
            if *changes == 0 {
                t!("compare.identical").to_string()
            } else {
                changes.to_string()
            },
        )],
        Some(CompareResult::Images { .. }) => Vec::new(),
        Some(CompareResult::Binary {
            left_size,
            right_size,
            identical,
        }) => vec![
            fact(
                "compare.fact.size",
                format!("{} / {}", file_size(*left_size), file_size(*right_size)),
            ),
            fact(
                "compare.fact.contents",
                t!(if *identical {
                    "compare.identical"
                } else {
                    "compare.different"
                })
                .to_string(),
            ),
        ],
        Some(CompareResult::Unreadable) => {
            vec![fact(
                "compare.fact.status",
                t!("compare.unreadable").to_string(),
            )]
        }
    }
}

fn message(text: String) -> AnyElement {
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .text_color(rgb(FG_MUTED))
        .child(text)
        .into_any_element()
}

fn side_image(path: &Path) -> impl IntoElement {
    div().w(gpui::relative(HALF)).h_full().child(PreviewImage {
        path: path.to_path_buf(),
        zoom: 1.0,
        pan: (0.0, 0.0),
        gestures: None,
    })
}

impl BrowserView {
    pub(super) fn with_compare_open(element: Div, cx: &mut Context<Self>) -> Div {
        element.on_action(
            cx.listener(|this, _: &CompareFiles, window, cx| this.open_compare(window, cx)),
        )
    }

    fn diff_lines(&mut self, range: Range<usize>) -> Vec<DiffLine> {
        let Some(Some(CompareResult::Text { rows, .. })) =
            self.compare.state.as_ref().map(|state| &state.result)
        else {
            return Vec::new();
        };
        range
            .filter_map(|index| rows.get(index).cloned())
            .map(|row| DiffLine { row })
            .collect()
    }

    fn compare_stage(&self, cx: &mut Context<Self>) -> AnyElement {
        let result = self
            .compare
            .state
            .as_ref()
            .and_then(|state| state.result.as_ref());
        match result {
            None => message(t!("compare.comparing").to_string()),
            Some(CompareResult::Text { rows, .. }) => uniform_list(
                "compare-lines",
                rows.len(),
                cx.processor(|this, range: Range<usize>, _window, _cx| this.diff_lines(range)),
            )
            .size_full()
            .p(px(QUICK_LOOK_STAGE_PAD))
            .text_color(rgb(FG))
            .track_scroll(&self.compare.scroll)
            .into_any_element(),
            Some(CompareResult::Images { left, right }) => div()
                .size_full()
                .flex()
                .child(side_image(left))
                .child(side_image(right))
                .into_any_element(),
            Some(CompareResult::Binary { identical, .. }) => message(
                t!(if *identical {
                    "compare.identical"
                } else {
                    "compare.different"
                })
                .to_string(),
            ),
            Some(CompareResult::Unreadable) => message(t!("compare.unreadable").to_string()),
        }
    }

    fn compare_panel(&self, window: &Window, cx: &mut Context<Self>) -> Option<QuickLookPanel> {
        let state = self.compare.state.as_ref()?;
        let viewport = window.viewport_size();
        let margin = px(QUICK_LOOK_MARGIN * DOUBLE);
        let view = cx.entity().downgrade();
        let on_close: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_compare(window, cx));
        });
        let title = t!(
            "compare.title",
            left = name_of(&state.left),
            right = name_of(&state.right)
        );
        let facts = facts_of(state.result.as_ref());
        Some(QuickLookPanel {
            header: QuickLookHeader {
                icon: IconGlyph::new(Icon::Dual, FG).into_any_element(),
                title: title.to_string().into(),
                hint: t!("compare.hint").to_string().into(),
                on_close,
            },
            stage: self.compare_stage(cx),
            facts: QuickLookFacts { facts },
            size: (
                px(QUICK_LOOK_WIDTH).min(viewport.width - margin),
                px(QUICK_LOOK_HEIGHT).min(viewport.height - margin),
            ),
        })
    }

    pub(super) fn compare_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let panel = self.compare_panel(window, cx)?;
        let viewport = window.viewport_size();
        let scrim = div()
            .id("compare")
            .track_focus(&self.compare.focus)
            .key_context(COMPARE_CONTEXT)
            .occlude()
            .w(viewport.width)
            .h(viewport.height)
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(SCRIM))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.close_compare(window, cx)),
            )
            .child(panel);
        let scrim = Self::with_compare_actions(scrim, cx);
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

    fn with_compare_actions(element: Stateful<Div>, cx: &mut Context<Self>) -> Stateful<Div> {
        element
            .on_action(
                cx.listener(|this, _: &CloseCompare, window, cx| this.close_compare(window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &CompareScrollDown, _, cx| this.scroll_compare(1.0, cx)),
            )
            .on_action(
                cx.listener(|this, _: &CompareScrollUp, _, cx| this.scroll_compare(-1.0, cx)),
            )
    }
}
