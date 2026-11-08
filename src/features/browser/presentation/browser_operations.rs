use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use gpui::{Context, SharedString};
use rust_i18n::t;

use crate::core::constants::operations::TOAST_DURATION_MS;
use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::helpers::format::item_count::item_count;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::browser::presentation::state::toast_model::ToastModel;
use crate::features::operations::domain::entities::clipboard::Clipboard;
use crate::features::operations::domain::entities::file_operation::{FileOperation, Transfer};
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::usecases::plan_transfer::plan_transfer;
use crate::features::operations::domain::usecases::run_operation::run_operation;
use crate::features::operations::domain::usecases::unique_target::unique_target;
use crate::features::places::domain::entities::place::PlaceTarget;

impl BrowserView {
    pub(super) fn copy_to_clipboard(&mut self, mode: ClipboardMode, cx: &mut Context<Self>) {
        let paths = self.active_pane().read(cx).target_paths();
        if paths.is_empty() {
            return;
        }
        let items = item_count(paths.len());
        let message = match mode {
            ClipboardMode::Copy => t!("op.clipboard_copy", items = items),
            ClipboardMode::Cut => t!("op.clipboard_cut", items = items),
        };
        let clipboard = Clipboard { mode, paths };
        let (system, published) = (self.setup.system_clipboard.clone(), clipboard.clone());
        cx.background_executor()
            .spawn(async move { system.write(&published) })
            .detach();
        self.clipboard = Some(clipboard);
        self.sync_cut(cx);
        self.show_toast(message.to_string(), false, cx);
    }

    pub(super) fn paste(&mut self, cx: &mut Context<Self>) {
        let system = self.setup.system_clipboard.clone();
        self.paste_task = Some(cx.spawn(async move |this, cx| {
            let external = cx
                .background_executor()
                .spawn(async move { system.read() })
                .await;
            let _ = this.update(cx, |this, cx| this.paste_contents(external, cx));
        }));
    }

    fn paste_contents(&mut self, external: Option<Clipboard>, cx: &mut Context<Self>) {
        let Some(clipboard) = external.or_else(|| self.clipboard.clone()) else {
            return self.show_toast(t!("op.clipboard_empty").to_string(), false, cx);
        };
        let Some(directory) = self.active_pane().read(cx).directory() else {
            return self.show_toast(t!("op.cannot_paste_here").to_string(), false, cx);
        };
        let files = self.setup.operations.clone();
        let exists = |path: &Path| files.exists(path);
        match plan_transfer(&clipboard.paths, &directory, clipboard.mode, &exists) {
            Ok(operation) if operation.item_count() == 0 => {}
            Ok(operation) => {
                if clipboard.mode == ClipboardMode::Cut {
                    self.clipboard = None;
                    self.sync_cut(cx);
                    let system = self.setup.system_clipboard.clone();
                    cx.background_executor()
                        .spawn(async move { system.clear() })
                        .detach();
                }
                self.execute(operation, OperationIntent::Fresh, cx);
            }
            Err(error) => self.show_error(error, cx),
        }
    }

    pub(super) fn trash_selection(&mut self, cx: &mut Context<Self>) {
        if self.location(cx) == PlaceTarget::Trash {
            return self.show_toast(t!("op.already_in_trash").to_string(), false, cx);
        }
        let paths = self.active_pane().read(cx).target_paths();
        if !paths.is_empty() {
            self.execute(FileOperation::Trash(paths), OperationIntent::Fresh, cx);
        }
    }

    pub(super) fn new_folder(&mut self, cx: &mut Context<Self>) {
        let Some(directory) = self.active_pane().read(cx).directory() else {
            return self.show_toast(t!("op.cannot_paste_here").to_string(), false, cx);
        };
        let files = self.setup.operations.clone();
        let name = t!("op.new_folder_name").to_string();
        let target = unique_target(&directory, &name, &|path: &Path| files.exists(path));
        self.execute(
            FileOperation::CreateFolder(target),
            OperationIntent::FreshThenRename,
            cx,
        );
    }

    pub(super) fn rename(&mut self, transfer: Transfer, cx: &mut Context<Self>) {
        if self.setup.operations.exists(&transfer.to) {
            return self.show_error(OperationError::AlreadyExists, cx);
        }
        self.execute(FileOperation::Rename(transfer), OperationIntent::Fresh, cx);
    }

    pub(super) fn undo(&mut self, cx: &mut Context<Self>) {
        if self.is_busy(cx) {
            return;
        }
        match self.history.take_undo() {
            Some(operation) => {
                self.execute(operation.inverse(), OperationIntent::Undo(operation), cx)
            }
            None => self.show_toast(t!("op.nothing_to_undo").to_string(), false, cx),
        }
    }

    pub(super) fn redo(&mut self, cx: &mut Context<Self>) {
        if self.is_busy(cx) {
            return;
        }
        match self.history.take_redo() {
            Some(operation) => {
                self.execute(operation.clone(), OperationIntent::Redo(operation), cx)
            }
            None => self.show_toast(t!("op.nothing_to_redo").to_string(), false, cx),
        }
    }

    fn is_busy(&mut self, cx: &mut Context<Self>) -> bool {
        let busy = self.operation_task.is_some();
        if busy {
            self.show_toast(t!("op.busy").to_string(), false, cx);
        }
        busy
    }

    fn execute(
        &mut self,
        operation: FileOperation,
        intent: OperationIntent,
        cx: &mut Context<Self>,
    ) {
        if matches!(
            intent,
            OperationIntent::Fresh | OperationIntent::FreshThenRename
        ) && self.is_busy(cx)
        {
            return;
        }
        let files = self.setup.operations.clone();
        self.operation_task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { run_operation(operation, files.as_ref()) })
                .await;
            let _ = this.update(cx, |this, cx| this.finish(intent, result, cx));
        }));
    }

    fn finish(
        &mut self,
        intent: OperationIntent,
        result: Result<FileOperation, OperationError>,
        cx: &mut Context<Self>,
    ) {
        self.operation_task = None;
        let done = match result {
            Ok(done) => done,
            Err(error) => {
                match intent {
                    OperationIntent::Undo(operation) => self.history.push_undo(operation),
                    OperationIntent::Redo(operation) => self.history.push_redo(operation),
                    OperationIntent::Fresh | OperationIntent::FreshThenRename => {}
                }
                self.show_error(error, cx);
                return self.reload_all(None, false, cx);
            }
        };
        let rename = intent == OperationIntent::FreshThenRename;
        let (message, can_undo, landing) = match intent {
            OperationIntent::Fresh | OperationIntent::FreshThenRename => {
                self.history.record(done.clone());
                (Self::describe(&done), true, done.landing_path())
            }
            OperationIntent::Undo(original) => {
                let message = t!("op.undid", what = Self::describe(&original)).to_string();
                let landing = original.inverse().landing_path();
                self.history.push_redo(original);
                (message, false, landing)
            }
            OperationIntent::Redo(_) => {
                self.history.push_undo(done.clone());
                (
                    t!("op.redid", what = Self::describe(&done)).to_string(),
                    false,
                    done.landing_path(),
                )
            }
        };
        self.show_toast(message, can_undo, cx);
        self.reload_all(landing, rename, cx);
    }

    fn describe(operation: &FileOperation) -> String {
        let items = item_count(operation.item_count());
        let name_of = |path: &PathBuf| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default()
        };
        match operation {
            FileOperation::Copy(_) => t!("op.copied", items = items),
            FileOperation::Move(_) => t!("op.moved", items = items),
            FileOperation::Rename(transfer) => t!("op.renamed", name = name_of(&transfer.to)),
            FileOperation::Trash(_) => t!("op.trashed", items = items),
            FileOperation::Restore(_) => t!("op.restored", items = items),
            FileOperation::CreateFolder(path) => t!("op.created_folder", name = name_of(path)),
            FileOperation::RemoveFolder(path) => t!("op.removed_folder", name = name_of(path)),
        }
        .to_string()
    }

    fn reload_all(&mut self, landing: Option<PathBuf>, rename: bool, cx: &mut Context<Self>) {
        let active = self.active;
        for (index, tab) in self.tabs.iter().enumerate() {
            let focus = landing.clone().filter(|_| index == active);
            tab.pane
                .update(cx, |pane, cx| pane.reload_with_focus(focus, rename, cx));
        }
    }

    pub(super) fn sync_cut(&mut self, cx: &mut Context<Self>) {
        let cut: HashSet<PathBuf> = match &self.clipboard {
            Some(clipboard) if clipboard.mode == ClipboardMode::Cut => {
                clipboard.paths.iter().cloned().collect()
            }
            _ => HashSet::new(),
        };
        for tab in &self.tabs {
            tab.pane
                .update(cx, |pane, cx| pane.set_cut(cut.clone(), cx));
        }
    }

    fn show_error(&mut self, error: OperationError, cx: &mut Context<Self>) {
        let reason = t!(error.label_key());
        self.show_toast(t!("op.failed", reason = reason).to_string(), false, cx);
    }

    pub(super) fn show_toast(&mut self, message: String, can_undo: bool, cx: &mut Context<Self>) {
        self.toast_generation += 1;
        let generation = self.toast_generation;
        self.toast = Some(ToastModel {
            message: SharedString::from(message),
            can_undo,
        });
        self.toast_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(TOAST_DURATION_MS))
                .await;
            let _ = this.update(cx, |this, cx| {
                if this.toast_generation == generation {
                    this.toast = None;
                    cx.notify();
                }
            });
        }));
        cx.notify();
    }
}
