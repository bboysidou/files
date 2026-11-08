use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use gpui::{Context, SharedString, Window};
use rust_i18n::t;

use crate::core::constants::operations::TOAST_DURATION_MS;
use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::enums::operations::collision_policy::CollisionPolicy;
use crate::core::helpers::format::item_count::item_count;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::confirm_state::ConfirmState;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::browser::presentation::state::toast_model::ToastModel;
use crate::features::operations::domain::entities::clipboard::Clipboard;
use crate::features::operations::domain::entities::file_operation::{FileOperation, Transfer};
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::entities::transfer_request::TransferRequest;
use crate::features::operations::domain::usecases::plan_transfer::{
    count_collisions, plan_transfer,
};
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
        self.jobs.paste_task = Some(cx.spawn(async move |this, cx| {
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
        let Some(destination) = self.active_pane().read(cx).directory() else {
            return self.show_toast(t!("op.cannot_paste_here").to_string(), false, cx);
        };
        let request = TransferRequest {
            sources: clipboard.paths,
            destination,
            mode: clipboard.mode,
            policy: CollisionPolicy::KeepBoth,
            in_place: false,
        };
        self.start_transfer(request, cx);
    }

    pub(super) fn start_transfer(&mut self, request: TransferRequest, cx: &mut Context<Self>) {
        let files = self.setup.operations.clone();
        let collisions = count_collisions(&request, &|path: &Path| files.exists(path));
        if collisions > 0 {
            self.confirm = Some(ConfirmState::Collision(request, collisions));
            self.focus_confirm = true;
            return cx.notify();
        }
        self.transfer(request, cx);
    }

    pub(super) fn transfer(&mut self, request: TransferRequest, cx: &mut Context<Self>) {
        let files = self.setup.operations.clone();
        match plan_transfer(&request, files.as_ref()) {
            Ok(operation) if operation.item_count() == 0 => {}
            Ok(operation) => {
                if request.mode == ClipboardMode::Cut {
                    self.clipboard = None;
                    self.sync_cut(cx);
                    let system = self.setup.system_clipboard.clone();
                    cx.background_executor()
                        .spawn(async move { system.clear() })
                        .detach();
                }
                let intent = if request.in_place {
                    OperationIntent::FreshInPlace
                } else {
                    OperationIntent::Fresh
                };
                self.execute(operation, intent, cx);
            }
            Err(error) => self.show_error(error, cx),
        }
    }

    pub(super) fn trash_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let paths = self.active_pane().read(cx).target_paths();
        if self.location(cx) == PlaceTarget::Trash {
            if !paths.is_empty() {
                self.ask(ConfirmState::Purge(paths), window, cx);
            }
            return;
        }
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
        if self.job_running() {
            return self.show_toast(t!("op.busy").to_string(), false, cx);
        }
        match self.history.take_undo() {
            Some(operation) => {
                self.execute(operation.inverse(), OperationIntent::Undo(operation), cx)
            }
            None => self.show_toast(t!("op.nothing_to_undo").to_string(), false, cx),
        }
    }

    pub(super) fn redo(&mut self, cx: &mut Context<Self>) {
        if self.job_running() {
            return self.show_toast(t!("op.busy").to_string(), false, cx);
        }
        match self.history.take_redo() {
            Some(operation) => {
                self.execute(operation.clone(), OperationIntent::Redo(operation), cx)
            }
            None => self.show_toast(t!("op.nothing_to_redo").to_string(), false, cx),
        }
    }

    pub(super) fn describe(operation: &FileOperation) -> String {
        if let FileOperation::Batch(steps) = operation {
            return steps.last().map(Self::describe).unwrap_or_default();
        }
        if let Some((key, path)) = named_step(operation) {
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            return t!(key, name = name).to_string();
        }
        let items = item_count(operation.item_count());
        match operation {
            FileOperation::Copy(_) => t!("op.copied", items = items),
            FileOperation::Move(_) => t!("op.moved", items = items),
            FileOperation::Trash(_) => t!("op.trashed", items = items),
            FileOperation::Restore(_) => t!("op.restored", items = items),
            FileOperation::Link(_) => t!("op.linked", items = items),
            _ => Default::default(),
        }
        .to_string()
    }

    pub(super) fn reload_everything(&mut self, cx: &mut Context<Self>) {
        self.reload_all(None, false, cx);
    }

    pub(super) fn reload_all(
        &mut self,
        landing: Option<PathBuf>,
        rename: bool,
        cx: &mut Context<Self>,
    ) {
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

    pub(super) fn show_error(&mut self, error: OperationError, cx: &mut Context<Self>) {
        let reason = t!(error.label_key());
        self.show_toast(t!("op.failed", reason = reason).to_string(), false, cx);
    }

    pub(super) fn show_toast(&mut self, message: String, can_undo: bool, cx: &mut Context<Self>) {
        self.toasts.generation += 1;
        let generation = self.toasts.generation;
        self.toasts.current = Some(ToastModel {
            message: SharedString::from(message),
            can_undo,
        });
        self.toasts.task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(TOAST_DURATION_MS))
                .await;
            let _ = this.update(cx, |this, cx| {
                if this.toasts.generation == generation {
                    this.toasts.current = None;
                    cx.notify();
                }
            });
        }));
        cx.notify();
    }

    pub(super) fn copy_paths(&mut self, cx: &mut Context<Self>) {
        let paths = self.active_pane().read(cx).target_paths();
        let Some(first) = paths.first() else {
            return;
        };
        let message = if paths.len() == 1 {
            t!("op.copied_path_one", path = first.to_string_lossy()).to_string()
        } else {
            t!("op.copied_paths", count = paths.len()).to_string()
        };
        let text = paths
            .iter()
            .map(|path| path.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\n");
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
        self.show_toast(message, false, cx);
    }
}

fn named_step(operation: &FileOperation) -> Option<(&'static str, &PathBuf)> {
    match operation {
        FileOperation::Rename(transfer) => Some(("op.renamed", &transfer.to)),
        FileOperation::CreateFolder(path) => Some(("op.created_folder", path)),
        FileOperation::RemoveFolder(path) => Some(("op.removed_folder", path)),
        FileOperation::Extract(path) => Some(("archive.extracted", path)),
        FileOperation::Compress(path) => Some(("archive.compressed", path)),
        FileOperation::CreateFile(path) => Some(("op.created_file", path)),
        FileOperation::SetMode(change) => Some(("op.mode_changed", &change.path)),
        _ => None,
    }
}
