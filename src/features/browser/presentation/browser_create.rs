use std::path::{Path, PathBuf};

use gpui::Context;
use rust_i18n::t;

use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::operations::domain::entities::clipboard::Clipboard;
use crate::features::operations::domain::entities::file_operation::{FileOperation, Transfer};
use crate::features::operations::domain::usecases::unique_target::unique_target;

impl BrowserView {
    pub(super) fn new_file(&mut self, cx: &mut Context<Self>) {
        let Some(directory) = self.active_pane().read(cx).directory() else {
            return self.show_toast(t!("op.cannot_paste_here").to_string(), false, cx);
        };
        let files = self.setup.operations.clone();
        let name = t!("op.new_file_name").to_string();
        let target = unique_target(&directory, &name, &|path: &Path| files.exists(path));
        self.execute(
            FileOperation::CreateFile(target),
            OperationIntent::FreshThenRename,
            cx,
        );
    }

    pub(super) fn duplicate(&mut self, cx: &mut Context<Self>) {
        let sources = self.active_pane().read(cx).target_paths();
        let transfers = self.beside(&sources);
        if transfers.is_empty() {
            return;
        }
        self.execute(FileOperation::Copy(transfers), OperationIntent::Fresh, cx);
    }

    pub(super) fn paste_links(&mut self, cx: &mut Context<Self>) {
        let system = self.setup.system_clipboard.clone();
        self.jobs.paste_task = Some(cx.spawn(async move |this, cx| {
            let external = cx
                .background_executor()
                .spawn(async move { system.read() })
                .await;
            let _ = this.update(cx, |this, cx| this.link_contents(external, cx));
        }));
    }

    fn link_contents(&mut self, external: Option<Clipboard>, cx: &mut Context<Self>) {
        let Some(clipboard) = external.or_else(|| self.clipboard.clone()) else {
            return self.show_toast(t!("op.clipboard_empty").to_string(), false, cx);
        };
        let Some(directory) = self.active_pane().read(cx).directory() else {
            return self.show_toast(t!("op.cannot_paste_here").to_string(), false, cx);
        };
        let files = self.setup.operations.clone();
        let transfers: Vec<Transfer> = clipboard
            .paths
            .into_iter()
            .filter_map(|from| {
                let name = from.file_name()?.to_string_lossy().into_owned();
                let to = unique_target(&directory, &name, &|path: &Path| {
                    files.exists(path) || path.symlink_metadata().is_ok()
                });
                Some(Transfer { from, to })
            })
            .collect();
        if !transfers.is_empty() {
            self.execute(FileOperation::Link(transfers), OperationIntent::Fresh, cx);
        }
    }

    fn beside(&self, sources: &[PathBuf]) -> Vec<Transfer> {
        let files = self.setup.operations.clone();
        sources
            .iter()
            .filter_map(|from| {
                let parent = from.parent()?;
                let name = from.file_name()?.to_string_lossy().into_owned();
                let to = unique_target(parent, &name, &|path: &Path| files.exists(path));
                Some(Transfer {
                    from: from.clone(),
                    to,
                })
            })
            .collect()
    }
}
