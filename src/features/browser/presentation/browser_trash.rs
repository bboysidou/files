use std::path::PathBuf;
use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Focusable, IntoElement, ParentElement, Window,
    anchored, deferred, point, px,
};
use rust_i18n::t;

use crate::core::enums::operations::collision_policy::CollisionPolicy;
use crate::core::enums::view::button_tone::ButtonTone;
use crate::core::helpers::format::item_count::item_count;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::key_bindings::CONFIRM_CONTEXT;
use crate::features::browser::presentation::state::confirm_state::ConfirmState;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::operations::domain::entities::file_operation::FileOperation;
use crate::global_components::confirm_dialog::ConfirmDialog;
use crate::global_components::dialog_button::DialogButton;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::modal_scrim::ModalScrim;

const CONFIRM_PRIORITY: usize = 3;

type Choice = fn(&mut BrowserView, &mut Window, &mut Context<BrowserView>);

struct ButtonSpec {
    id: &'static str,
    label_key: &'static str,
    danger: bool,
    choice: Choice,
}

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

    fn take_confirm(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<ConfirmState> {
        let request = self.confirm.take();
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
        request
    }

    fn resolve(&mut self, policy: CollisionPolicy, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ConfirmState::Collision(mut request, _)) = self.take_confirm(window, cx) {
            request.policy = policy;
            self.transfer(request, cx);
        }
    }

    fn keep_both(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.resolve(CollisionPolicy::KeepBoth, window, cx);
    }

    fn skip(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.resolve(CollisionPolicy::Skip, window, cx);
    }

    fn replace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.resolve(CollisionPolicy::Replace, window, cx);
    }

    pub(super) fn accept_confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.confirm, Some(ConfirmState::Collision(..))) {
            return self.keep_both(window, cx);
        }
        let Some(request) = self.take_confirm(window, cx) else {
            return;
        };
        let bin = self.setup.trash_bin.clone();
        let message = match &request {
            ConfirmState::Purge(entries) => {
                t!("op.purged", items = item_count(entries.len())).to_string()
            }
            _ => t!("op.emptied").to_string(),
        };
        self.trash_task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    match request {
                        ConfirmState::Purge(entries) => bin.purge(&entries),
                        _ => bin.empty(),
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
    }

    fn button(&self, spec: ButtonSpec, cx: &mut Context<Self>) -> DialogButton {
        let view = cx.entity().downgrade();
        let choice = spec.choice;
        let on_click: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| choice(this, window, cx));
        });
        DialogButton {
            id: spec.id,
            label: t!(spec.label_key).to_string().into(),
            tone: if spec.danger {
                ButtonTone::Danger
            } else {
                ButtonTone::Plain
            },
            on_click,
        }
    }

    fn cancel_button(&self, cx: &mut Context<Self>) -> DialogButton {
        self.button(
            ButtonSpec {
                id: "confirm-cancel",
                label_key: "confirm.cancel",
                danger: false,
                choice: Self::cancel_confirm,
            },
            cx,
        )
    }

    fn collision_dialog(&self, count: usize, cx: &mut Context<Self>) -> ConfirmDialog {
        let buttons = vec![
            self.cancel_button(cx),
            self.button(
                ButtonSpec {
                    id: "collision-skip",
                    label_key: "collision.skip",
                    danger: false,
                    choice: Self::skip,
                },
                cx,
            ),
            self.button(
                ButtonSpec {
                    id: "collision-replace",
                    label_key: "collision.replace",
                    danger: false,
                    choice: Self::replace,
                },
                cx,
            ),
            self.button(
                ButtonSpec {
                    id: "collision-keep",
                    label_key: "collision.keep",
                    danger: true,
                    choice: Self::keep_both,
                },
                cx,
            ),
        ];
        ConfirmDialog {
            title: if count == 1 {
                t!("collision.title_one").to_string().into()
            } else {
                t!("collision.title_other", count = count)
                    .to_string()
                    .into()
            },
            body: t!("collision.body").to_string().into(),
            buttons,
        }
    }

    fn destructive_dialog(
        &self,
        title: String,
        body: String,
        cx: &mut Context<Self>,
    ) -> ConfirmDialog {
        let delete = ButtonSpec {
            id: "confirm-accept",
            label_key: "confirm.delete",
            danger: true,
            choice: Self::accept_confirm,
        };
        ConfirmDialog {
            title: title.into(),
            body: body.into(),
            buttons: vec![self.cancel_button(cx), self.button(delete, cx)],
        }
    }

    fn dialog(&self, request: &ConfirmState, cx: &mut Context<Self>) -> ConfirmDialog {
        match request {
            ConfirmState::Collision(_, count) => self.collision_dialog(*count, cx),
            ConfirmState::Purge(entries) => self.destructive_dialog(
                t!("confirm.purge.title", items = item_count(entries.len())).to_string(),
                t!("confirm.purge.body").to_string(),
                cx,
            ),
            ConfirmState::EmptyTrash => self.destructive_dialog(
                t!("confirm.empty.title").to_string(),
                t!("confirm.empty.body").to_string(),
                cx,
            ),
        }
    }

    pub(super) fn confirm_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let request = self.confirm.clone()?;
        let view = cx.entity().downgrade();
        let on_dismiss: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.cancel_confirm(window, cx));
        });
        let scrim = ModalScrim {
            focus: self.confirm_focus.clone(),
            context: CONFIRM_CONTEXT,
            viewport: window.viewport_size(),
            on_dismiss,
            panel: self.dialog(&request, cx).into_any_element(),
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
