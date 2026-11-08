use gpui::{Context, Window};
use rust_i18n::t;

use crate::core::helpers::files::archive_stem::archive_stem;
use crate::core::helpers::files::is_archive::is_archive;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::archive_job::ArchiveJob;
use crate::features::operations::domain::entities::file_operation::FileOperation;
use crate::features::operations::domain::usecases::unique_target::unique_target;

impl BrowserView {
    pub(super) fn extract_here(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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
        let (tool, archive, done) = (
            self.setup.archives.clone(),
            entry.path.clone(),
            destination.clone(),
        );
        let job = ArchiveJob {
            title_key: "job.extracting",
            work: Box::new(move |counter| tool.extract(&archive, &destination, counter)),
            done: Box::new(move |this, _, cx| {
                let message = Self::describe(&FileOperation::Extract(done.clone()));
                this.history.record(FileOperation::Extract(done));
                this.show_toast(message, true, cx);
            }),
        };
        self.run_archive_job(job, window, cx);
    }
}
