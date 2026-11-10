use std::sync::Arc;
use std::time::Instant;

use gpui::{Context, Window};
use rust_i18n::t;

use crate::features::archives::domain::entities::archive_error::ArchiveError;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::archive_job::ArchiveJob;
use crate::features::browser::presentation::state::job_note::JobNote;
use crate::features::browser::presentation::state::job_state::JobState;
use crate::features::browser::presentation::state::note_outcome::NoteOutcome;
use crate::features::operations::domain::entities::job_counter::JobCounter;

impl BrowserView {
    pub(super) fn run_archive_job(
        &mut self,
        job: ArchiveJob,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.job_running() {
            return self.show_toast(t!("op.busy").to_string(), false, cx);
        }
        let ArchiveJob {
            title_key,
            paths,
            work,
            done,
        } = job;
        let counter = Arc::new(JobCounter::default());
        self.jobs.job = Some(JobState {
            counter: counter.clone(),
            title_key,
            started: Instant::now(),
            note: JobNote::default(),
            paths: Self::resolve_job_paths(paths),
        });
        self.jobs.archive_task = Some(cx.spawn_in(window, async move |this, cx| {
            let outcome = cx
                .background_executor()
                .spawn(async move { work(counter.as_ref()) })
                .await;
            let _ = this.update_in(cx, |this, window, cx| {
                this.jobs.archive_task = None;
                let job = this.jobs.job.take();
                Self::note_archive(job, &outcome);
                match outcome {
                    Ok(()) => done(this, window, cx),
                    Err(ArchiveError::Cancelled) => {
                        this.show_toast(t!("op.error.cancelled").to_string(), false, cx)
                    }
                    Err(ArchiveError::Failed(reason)) => this.show_toast(
                        t!("archive.failed", reason = reason).to_string(),
                        false,
                        cx,
                    ),
                }
                this.reload_everything(cx);
                if let Some((operation, intent)) = this.jobs.queue.pop_front() {
                    this.execute(operation, intent, cx);
                }
            });
        }));
        self.start_job_ticker(cx);
    }

    fn note_archive(job: Option<JobState>, outcome: &Result<(), ArchiveError>) {
        let verb = job
            .as_ref()
            .map(|job| t!(format!("note.{}", job.title_key)).to_string())
            .unwrap_or_default();
        let result = match outcome {
            Ok(()) => NoteOutcome::Done(t!("note.job_finished", what = verb).to_string()),
            Err(ArchiveError::Cancelled) => NoteOutcome::Cancelled,
            Err(ArchiveError::Failed(reason)) => NoteOutcome::Failed(reason.clone()),
        };
        Self::note_finish(job, result);
    }
}
