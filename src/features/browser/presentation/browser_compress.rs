use std::path::Path;

use gpui::{Context, Window};
use rust_i18n::t;

use crate::core::enums::archives::archive_format::ArchiveFormat;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::archive_job::ArchiveJob;
use crate::features::operations::domain::entities::file_operation::FileOperation;
use crate::features::operations::domain::usecases::unique_target::unique_target;

impl BrowserView {
    pub(super) fn compress(
        &mut self,
        format: ArchiveFormat,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let sources = self.active_pane().read(cx).target_paths();
        let Some(parent) = sources
            .first()
            .and_then(|first| first.parent())
            .map(Path::to_path_buf)
        else {
            return;
        };
        if sources
            .iter()
            .any(|source| source.parent() != Some(parent.as_path()))
        {
            return self.show_toast(t!("archive.one_folder").to_string(), false, cx);
        }
        let base = match sources.as_slice() {
            [single] => single.file_name(),
            _ => parent.file_name(),
        }
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| t!("archive.default_name").to_string());
        let target = unique_target(&parent, &format!("{base}{}", format.suffix()), &|path| {
            path.exists()
        });
        let (tool, done) = (self.setup.archives.clone(), target.clone());
        let job = ArchiveJob {
            title_key: "job.compressing",
            paths: sources.iter().cloned().chain([target.clone()]).collect(),
            work: Box::new(move |counter| tool.compress(&sources, &target, counter)),
            done: Box::new(move |this, _, cx| {
                let message = Self::describe(&FileOperation::Compress(done.clone()));
                this.history.record(FileOperation::Compress(done));
                this.show_toast(message, true, cx);
            }),
        };
        self.run_archive_job(job, window, cx);
    }
}
