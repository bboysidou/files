use gpui::Context;
use rust_i18n::t;

use crate::core::helpers::files::archive_stem::archive_stem;
use crate::core::helpers::files::is_archive::is_archive;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::operations::domain::entities::file_operation::FileOperation;
use crate::features::operations::domain::usecases::unique_target::unique_target;

impl BrowserView {
    pub(super) fn extract_here(&mut self, cx: &mut Context<Self>) {
        let Some(entry) = self
            .active_pane()
            .read(cx)
            .current_entry()
            .filter(is_archive)
        else {
            return self.show_toast(t!("archive.not_archive").to_string(), false, cx);
        };
        let Some(parent) = entry.path.parent() else {
            return;
        };
        let destination = unique_target(parent, archive_stem(&entry.name), &|path| path.exists());
        let extractor = self.setup.extractor.clone();
        let (archive, done) = (entry.path.clone(), destination.clone());
        let name = destination
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default();
        self.show_toast(
            t!("archive.extracting", name = entry.name).to_string(),
            false,
            cx,
        );
        cx.spawn(async move |this, cx| {
            let outcome = cx
                .background_executor()
                .spawn(async move { extractor.extract(&archive, &destination) })
                .await;
            let _ = this.update(cx, |this, cx| match outcome {
                Ok(()) => {
                    this.history.record(FileOperation::Extract(done));
                    let message = t!("archive.extracted", name = name).to_string();
                    this.show_toast(message, true, cx);
                }
                Err(reason) => {
                    let message = t!("archive.failed", reason = reason).to_string();
                    this.show_toast(message, false, cx);
                }
            });
        })
        .detach();
    }
}
