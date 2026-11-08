use std::path::PathBuf;
use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Focusable, IntoElement, ParentElement, Window,
    anchored, deferred, point, px,
};
use rust_i18n::t;

use crate::core::helpers::format::item_count::item_count;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::key_bindings::CONFIRM_CONTEXT;
use crate::features::browser::presentation::state::confirm_state::ConfirmState;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::operations::domain::entities::file_operation::FileOperation;
use crate::global_components::confirm_dialog::ConfirmDialog;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::modal_scrim::ModalScrim;

const CONFIRM_PRIORITY: usize = 3;

impl BrowserView {
    pub(super) fn restore_from_trash(&mut self, entries: Vec<PathBuf>, cx: &mut Context<Self>) {
        let bin = self.setup.trash_bin.clone();
        self.trash_task = Some(cx.spawn(async move |this, cx| {
            let originals = cx
                .background_executor()
                .spawn(async move { bin.originals(&entries) })
                .await;
            let _ = this.update(cx, |this, cx| {
                if !originals.is_empty() {
                    this.execute(
                        FileOperation::Restore(originals),
                        OperationIntent::Fresh,
                        cx,
                    );
                }
            });
        }));
    }

    pub(super) fn ask(
        &mut self,
        request: ConfirmState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.confirm = Some(request);
        window.focus(&self.confirm_focus, cx);
        cx.notify();
    }

    pub(super) fn cancel_confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.confirm = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn accept_confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(request) = self.confirm.take() else {
            return;
        };
        window.focus(&self.active_pane().focus_handle(cx), cx);
        let bin = self.setup.trash_bin.clone();
        let message = match &request {
            ConfirmState::Purge(entries) => {
                t!("op.purged", items = item_count(entries.len())).to_string()
            }
            ConfirmState::EmptyTrash => t!("op.emptied").to_string(),
        };
        self.trash_task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    match request {
                        ConfirmState::Purge(entries) => bin.purge(&entries),
                        ConfirmState::EmptyTrash => bin.empty(),
                    }
                })
                .await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(()) => this.show_toast(message, false, cx),
                    Err(error) => this.show_error(error, cx),
                }
                this.reload_everything(cx);
            });
        }));
        cx.notify();
    }

    fn confirm_text(request: &ConfirmState) -> (String, String) {
        match request {
            ConfirmState::Purge(entries) => (
                t!("confirm.purge.title", items = item_count(entries.len())).to_string(),
                t!("confirm.purge.body").to_string(),
            ),
            ConfirmState::EmptyTrash => (
                t!("confirm.empty.title").to_string(),
                t!("confirm.empty.body").to_string(),
            ),
        }
    }

    pub(super) fn confirm_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let request = self.confirm.as_ref()?;
        let (title, body) = Self::confirm_text(request);
        let (cancel_view, accept_view) = (cx.entity().downgrade(), cx.entity().downgrade());
        let on_cancel: ClickHandler = Rc::new(move |window, cx| {
            let _ = cancel_view.update(cx, |this, cx| this.cancel_confirm(window, cx));
        });
        let on_confirm: ClickHandler = Rc::new(move |window, cx| {
            let _ = accept_view.update(cx, |this, cx| this.accept_confirm(window, cx));
        });
        let dialog = ConfirmDialog {
            title: title.into(),
            body: body.into(),
            cancel_label: t!("confirm.cancel").to_string().into(),
            confirm_label: t!("confirm.delete").to_string().into(),
            on_cancel: on_cancel.clone(),
            on_confirm,
        };
        let scrim = ModalScrim {
            focus: self.confirm_focus.clone(),
            context: CONFIRM_CONTEXT,
            viewport: window.viewport_size(),
            on_dismiss: on_cancel,
            panel: dialog.into_any_element(),
        };
        let overlay = anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.0), px(0.0)))
            .child(scrim);
        Some(
            deferred(overlay)
                .with_priority(CONFIRM_PRIORITY)
                .into_any_element(),
        )
    }
}
