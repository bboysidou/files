use std::fs;
use std::os::unix::fs::MetadataExt;
use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Focusable, FontWeight, IntoElement, KeyDownEvent,
    MouseButton, Pixels, Size, Window, anchored, deferred, div, point, prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::constants::files::{
    MISSING_VALUE, PERMISSION_BITS, PERMISSION_COUNT, PERMISSION_TOP_BIT,
};
use crate::core::helpers::files::device_tree_size::device_tree_size;
use crate::core::helpers::format::file_size::file_size;
use crate::core::helpers::time::absolute_time::absolute_time;
use crate::core::theme::colors::{CARD, FG_BRIGHT, LINE_STRONG};
use crate::core::theme::metrics::{
    QUICK_LOOK_MARGIN, QUICK_LOOK_RADIUS, SETTINGS_PANEL_WIDTH, SHEET_PANEL_PAD, TEXT_EMPTY_TITLE,
    TOOLBAR_GAP,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::permission_grid::PermissionGrid;
use crate::features::browser::presentation::components::settings_row::SettingsRow;
use crate::features::browser::presentation::key_bindings::PROPERTIES_CONTEXT;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::browser::presentation::state::properties_state::PropertiesState;
use crate::features::operations::domain::entities::file_operation::{FileOperation, ModeChange};
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::modal_scrim::ModalScrim;

const PROPERTIES_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;
const OCTAL_PREFIX: &str = "0";

impl BrowserView {
    pub(super) fn open_properties(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self.active_pane().read(cx).current_entry() else {
            return;
        };
        let Some(props) = self.setup.properties.read(&entry.path) else {
            return;
        };
        let task = props.is_dir.then(|| {
            let folder = props.path.clone();
            cx.spawn(async move |this, cx| {
                let size = cx
                    .background_executor()
                    .spawn(async move {
                        let device = fs::metadata(&folder)
                            .map(|meta| meta.dev())
                            .unwrap_or_default();
                        device_tree_size(&folder, device)
                    })
                    .await;
                let _ = this.update(cx, |this, cx| {
                    if let Some(state) = this.properties.as_mut() {
                        state.folder_size = Some(size);
                        cx.notify();
                    }
                });
            })
        });
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        self.properties = Some(PropertiesState {
            kind: t!(entry.kind.label_key()).to_string().into(),
            props,
            folder_size: None,
            focus,
            _task: task,
        });
        cx.notify();
    }

    pub(super) fn close_properties(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.properties = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn toggle_permission(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(state) = self.properties.as_mut() else {
            return;
        };
        if index >= PERMISSION_COUNT {
            return;
        }
        let from = state.props.mode;
        let to = from ^ (PERMISSION_TOP_BIT >> index);
        state.props.mode = to;
        let change = ModeChange {
            path: state.props.path.clone(),
            from,
            to,
        };
        self.execute(FileOperation::SetMode(change), OperationIntent::Fresh, cx);
    }

    fn properties_rows(state: &PropertiesState) -> Vec<(&'static str, String)> {
        let props = &state.props;
        let size = match (props.is_dir, state.folder_size) {
            (true, Some(bytes)) | (false, Some(bytes)) => file_size(bytes),
            (true, None) => t!("props.measuring").to_string(),
            (false, None) => file_size(props.size),
        };
        let mut rows = vec![
            ("props.kind", state.kind.to_string()),
            (
                "props.location",
                props
                    .path
                    .parent()
                    .map(|parent| parent.display().to_string())
                    .unwrap_or_else(|| MISSING_VALUE.to_owned()),
            ),
            ("props.size", size),
            ("props.modified", absolute_time(props.modified)),
            ("props.accessed", absolute_time(props.accessed)),
            ("props.created", absolute_time(props.created)),
            ("props.owner", format!("{} · {}", props.owner, props.group)),
            (
                "props.mode",
                format!("{OCTAL_PREFIX}{:o}", props.mode & PERMISSION_BITS),
            ),
        ];
        if let Some(target) = &props.link_target {
            rows.push(("props.link", target.display().to_string()));
        }
        rows
    }

    fn properties_panel(&self, viewport: Size<Pixels>, cx: &Context<Self>) -> Option<AnyElement> {
        let state = self.properties.as_ref()?;
        let props = &state.props;
        let view = cx.entity().downgrade();
        let on_toggle: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = view.update(cx, |this, cx| this.toggle_permission(index, cx));
        });
        let rows = Self::properties_rows(state)
            .into_iter()
            .map(|(label, value)| SettingsRow {
                label: t!(label).to_string().into(),
                control: div()
                    .text_color(rgb(FG_BRIGHT))
                    .child(value)
                    .into_any_element(),
            });
        let name = props
            .path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let margin = px(QUICK_LOOK_MARGIN * DOUBLE);
        Some(
            div()
                .flex()
                .flex_col()
                .gap(px(TOOLBAR_GAP))
                .w(px(SETTINGS_PANEL_WIDTH).min(viewport.width - margin))
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
                        .child(name),
                )
                .children(rows)
                .child(PermissionGrid {
                    mode: props.mode,
                    on_toggle,
                })
                .child(div().child(t!("props.hint").to_string()))
                .into_any_element(),
        )
    }

    pub(super) fn properties_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let viewport = window.viewport_size();
        let panel = self.properties_panel(viewport, cx)?;
        let focus = self.properties.as_ref()?.focus.clone();
        let view = cx.entity().downgrade();
        let on_dismiss: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_properties(window, cx));
        });
        let scrim = ModalScrim {
            focus,
            context: PROPERTIES_CONTEXT,
            viewport,
            on_dismiss,
            panel,
        };
        let keys = div()
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                let digit = event
                    .keystroke
                    .key
                    .parse::<usize>()
                    .ok()
                    .filter(|digit| *digit > 0);
                if let Some(digit) = digit {
                    this.toggle_permission(digit - 1, cx);
                }
            }))
            .child(scrim);
        let overlay = anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.0), px(0.0)))
            .child(keys);
        Some(
            deferred(overlay)
                .with_priority(PROPERTIES_PRIORITY)
                .into_any_element(),
        )
    }
}
