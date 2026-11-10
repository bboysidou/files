use std::path::{Path, PathBuf};

use rust_i18n::t;

use crate::core::constants::notifications::NOTE_PERCENT_MAX;
use crate::core::helpers::files::resolved_path::resolved_path;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::job_on_target::JobOnTarget;

impl BrowserView {
    pub fn resolve_job_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
        paths.iter().map(|path| resolved_path(path)).collect()
    }

    pub fn job_on(&self, root: &Path) -> Option<JobOnTarget> {
        let job = self.jobs.job.as_ref()?;
        let root = resolved_path(root);
        if !job.paths.iter().any(|path| path.starts_with(&root)) {
            return None;
        }
        let (done, total) = (job.counter.done(), job.counter.total());
        Some(JobOnTarget {
            what: t!(format!("note.{}", job.title_key)).to_string(),
            percent: (done.min(total) * NOTE_PERCENT_MAX)
                .checked_div(total)
                .unwrap_or(0) as u8,
        })
    }
}
