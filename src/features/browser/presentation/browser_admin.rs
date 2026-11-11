use std::rc::Rc;

use gpui::{Context, SharedString};
use rust_i18n::t;

use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::browser::presentation::state::toast_model::ToastAction;
use crate::features::operations::domain::entities::file_operation::FileOperation;
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::global_components::handlers::ClickHandler;

pub type AdminRetry = Option<(FileOperation, OperationIntent)>;

impl BrowserView {
    pub(super) fn wants_admin(&self, error: OperationError) -> bool {
        error == OperationError::PermissionDenied && !self.setup.admin.is_active()
    }

    pub(super) fn authenticate(&mut self, retry: AdminRetry, cx: &mut Context<Self>) {
        let admin = self.setup.admin.clone();
        cx.spawn(async move |this, cx| {
            let started = cx
                .background_executor()
                .spawn(async move { admin.start() })
                .await;
            let _ = this.update(cx, |this, cx| match (started, retry) {
                (Ok(()), Some((operation, intent))) => this.execute(operation, intent, cx),
                (Ok(()), None) => this.reload_everything(cx),
                (Err(error), _) => this.show_toast(t!(error.label_key()).to_string(), false, cx),
            });
        })
        .detach();
    }

    pub(super) fn offer_admin(
        &mut self,
        error: OperationError,
        retry: AdminRetry,
        cx: &mut Context<Self>,
    ) {
        let view = cx.entity().downgrade();
        let on_click: ClickHandler = Rc::new(move |_, cx| {
            let retry = retry.clone();
            let _ = view.update(cx, |this, cx| this.authenticate(retry, cx));
        });
        let message = t!("op.failed", reason = t!(error.label_key())).to_string();
        let label = SharedString::from(t!("auth.authenticate").to_string());
        self.show_toast_action(message, Some(ToastAction::Run(label, on_click)), cx);
    }

    pub(super) fn end_admin_handler(&self, cx: &mut Context<Self>) -> Option<ClickHandler> {
        if !self.setup.admin.is_active() {
            return None;
        }
        let view = cx.entity().downgrade();
        Some(Rc::new(move |_, cx| {
            let _ = view.update(cx, |this, cx| this.end_admin(cx));
        }))
    }

    fn end_admin(&mut self, cx: &mut Context<Self>) {
        let admin = self.setup.admin.clone();
        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .spawn(async move { admin.end() })
                .await;
            let _ = this.update(cx, |this, cx| {
                this.show_toast(t!("admin.ended").to_string(), false, cx);
                this.reload_everything(cx);
            });
        })
        .detach();
    }
}
