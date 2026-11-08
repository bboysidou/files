use std::path::{Path, PathBuf};
use std::rc::Rc;

use gpui::Focusable;
use gpui::{
    AnchoredPositionMode, AnyElement, App, Context, Entity, FontWeight, IntoElement, MouseButton,
    Pixels, Size, Subscription, Window, anchored, deferred, div, point, prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::constants::operations::BULK_PREVIEW_LIMIT;
use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT, FG_MUTED, LINE_STRONG};
use crate::core::theme::metrics::{
    CONTROL_RADIUS, CRUMBS_HEIGHT, FILTER_FIELD_PAD_X, QUICK_LOOK_MARGIN, QUICK_LOOK_RADIUS,
    SETTINGS_PANEL_WIDTH, SHEET_PANEL_HEIGHT, SHEET_PANEL_PAD, TEXT_EMPTY_TITLE, TOOLBAR_GAP,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::bulk_row::BulkRow;
use crate::features::browser::presentation::key_bindings::BULK_CONTEXT;
use crate::features::browser::presentation::state::bulk_rename_state::{BulkPlan, BulkRenameState};
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::operations::domain::entities::file_operation::{FileOperation, Transfer};
use crate::features::operations::domain::usecases::bulk_conflicts::bulk_conflicts;
use crate::features::operations::domain::usecases::plan_bulk_rename::plan_bulk_rename;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::modal_scrim::ModalScrim;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};

const BULK_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;

impl BrowserView {
    pub(super) fn open_bulk_rename(
        &mut self,
        sources: Vec<PathBuf>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (find, find_events) = Self::bulk_field("bulk.find", window, cx);
        let (replace, replace_events) = Self::bulk_field("bulk.replace", window, cx);
        window.focus(&find.focus_handle(cx), cx);
        self.bulk = Some(BulkRenameState {
            sources,
            find,
            replace,
            focus: cx.focus_handle(),
            _events: [find_events, replace_events],
        });
        cx.notify();
    }

    fn bulk_field(
        key: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<TextField>, Subscription) {
        let field = cx.new(|cx| TextField::new(t!(key).to_string(), cx));
        let events = cx.subscribe_in(&field, window, |this, _, event, window, cx| match event {
            TextFieldEvent::Changed => cx.notify(),
            TextFieldEvent::Submitted => this.apply_bulk_rename(window, cx),
            TextFieldEvent::Cancelled => this.close_bulk_rename(window, cx),
        });
        (field, events)
    }

    pub(super) fn close_bulk_rename(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.bulk = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn switch_bulk_field(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(bulk) = self.bulk.as_ref() else {
            return;
        };
        let next = if bulk.find.focus_handle(cx).is_focused(window) {
            bulk.replace.focus_handle(cx)
        } else {
            bulk.find.focus_handle(cx)
        };
        window.focus(&next, cx);
    }

    fn bulk_plan(&self, cx: &App) -> Option<BulkPlan> {
        let bulk = self.bulk.as_ref()?;
        let old: Vec<String> = bulk
            .sources
            .iter()
            .map(|path| {
                path.file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default()
            })
            .collect();
        let new = plan_bulk_rename(
            &old,
            bulk.find.read(cx).text(),
            bulk.replace.read(cx).text(),
        );
        let files = self.setup.operations.clone();
        let conflicts = bulk_conflicts(&bulk.sources, &new, &|path: &Path| files.exists(path));
        Some(BulkPlan {
            old,
            new,
            conflicts,
        })
    }

    fn apply_bulk_rename(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(plan) = self.bulk_plan(cx) else {
            return;
        };
        if plan.conflicts.iter().any(|conflict| *conflict) {
            return self.show_toast(t!("bulk.conflict").to_string(), false, cx);
        }
        let sources = self
            .bulk
            .as_ref()
            .map(|bulk| bulk.sources.clone())
            .unwrap_or_default();
        let renames: Vec<FileOperation> = sources
            .into_iter()
            .zip(plan.old.iter().zip(&plan.new))
            .filter(|(_, (old, new))| old != new)
            .filter_map(|(from, (_, new))| {
                let to = from.parent()?.join(new);
                Some(FileOperation::Rename(Transfer { from, to }))
            })
            .collect();
        self.close_bulk_rename(window, cx);
        if !renames.is_empty() {
            self.execute(FileOperation::Batch(renames), OperationIntent::Fresh, cx);
        }
    }

    fn bulk_rows(&self, cx: &App) -> Vec<AnyElement> {
        let Some(plan) = self.bulk_plan(cx) else {
            return Vec::new();
        };
        plan.old
            .into_iter()
            .zip(plan.new)
            .zip(plan.conflicts)
            .take(BULK_PREVIEW_LIMIT)
            .map(|((old, new), conflict)| {
                BulkRow {
                    old: old.into(),
                    new: new.into(),
                    conflict,
                }
                .into_any_element()
            })
            .collect()
    }

    pub(super) fn dialog_input(label: String, field: Entity<TextField>) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(TOOLBAR_GAP / DOUBLE))
            .child(div().text_color(rgb(FG_MUTED)).child(label))
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(CRUMBS_HEIGHT))
                    .px(px(FILTER_FIELD_PAD_X))
                    .rounded(px(CONTROL_RADIUS))
                    .border_1()
                    .border_color(rgb(ACCENT))
                    .text_color(rgb(FG_BRIGHT))
                    .child(field),
            )
            .into_any_element()
    }

    fn bulk_panel(&self, viewport: Size<Pixels>, cx: &Context<Self>) -> Option<AnyElement> {
        let bulk = self.bulk.as_ref()?;
        let margin = px(QUICK_LOOK_MARGIN * DOUBLE);
        let title = t!("bulk.title", count = bulk.sources.len()).to_string();
        Some(
            div()
                .flex()
                .flex_col()
                .gap(px(TOOLBAR_GAP))
                .w(px(SETTINGS_PANEL_WIDTH).min(viewport.width - margin))
                .h(px(SHEET_PANEL_HEIGHT).min(viewport.height - margin))
                .p(px(SHEET_PANEL_PAD))
                .rounded(px(QUICK_LOOK_RADIUS))
                .bg(rgb(CARD))
                .border_1()
                .border_color(rgb(LINE_STRONG))
                .shadow_lg()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(
                    div()
                        .text_size(px(TEXT_EMPTY_TITLE))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(FG_BRIGHT))
                        .child(title),
                )
                .child(Self::dialog_input(
                    t!("bulk.find_label").to_string(),
                    bulk.find.clone(),
                ))
                .child(Self::dialog_input(
                    t!("bulk.replace_label").to_string(),
                    bulk.replace.clone(),
                ))
                .child(
                    div()
                        .text_color(rgb(FG_MUTED))
                        .child(t!("bulk.hint").to_string()),
                )
                .child(
                    div()
                        .id("bulk-rows")
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_h_0()
                        .overflow_y_scroll()
                        .children(self.bulk_rows(cx)),
                )
                .into_any_element(),
        )
    }

    pub(super) fn bulk_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let viewport = window.viewport_size();
        let panel = self.bulk_panel(viewport, cx)?;
        let focus = self.bulk.as_ref()?.focus.clone();
        let view = cx.entity().downgrade();
        let on_dismiss: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_bulk_rename(window, cx));
        });
        let scrim = ModalScrim {
            focus,
            context: BULK_CONTEXT,
            viewport,
            on_dismiss,
            panel,
        };
        let overlay = anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.0), px(0.0)))
            .child(scrim);
        Some(
            deferred(overlay)
                .with_priority(BULK_PRIORITY)
                .into_any_element(),
        )
    }
}
