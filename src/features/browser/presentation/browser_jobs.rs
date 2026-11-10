use std::sync::Arc;
use std::time::{Duration, Instant};

use gpui::{Context, SharedString};
use rust_i18n::t;

use crate::core::constants::operations::{JOB_CARD_DELAY_MS, JOB_TICK_MS};
use crate::core::enums::operations::job_phase::JobPhase;
use crate::core::helpers::format::file_size::file_size;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::job_card_model::JobCardModel;
use crate::features::browser::presentation::state::job_state::JobState;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::operations::domain::entities::file_operation::FileOperation;
use crate::features::operations::domain::entities::job_counter::JobCounter;
use crate::features::operations::domain::entities::operation_error::OperationError;
use crate::features::operations::domain::usecases::run_operation::run_operation;

impl BrowserView {
    pub(super) fn job_running(&self) -> bool {
        self.jobs.operation_task.is_some() || self.jobs.archive_task.is_some()
    }

    fn title_key(operation: &FileOperation) -> &'static str {
        match operation {
            FileOperation::Copy(_) => "job.copying",
            FileOperation::Move(_) => "job.moving",
            FileOperation::Batch(steps) => steps.last().map_or("job.working", Self::title_key),
            _ => "job.working",
        }
    }

    pub(super) fn execute(
        &mut self,
        operation: FileOperation,
        intent: OperationIntent,
        cx: &mut Context<Self>,
    ) {
        if self.job_running() {
            self.jobs.queue.push_back((operation, intent));
            return self.show_toast(t!("job.queued").to_string(), false, cx);
        }
        let counter = Arc::new(JobCounter::default());
        self.jobs.job = Some(JobState {
            counter: counter.clone(),
            title_key: Self::title_key(&operation),
            started: Instant::now(),
        });
        let files = self.setup.operations.clone();
        self.jobs.operation_task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { run_operation(operation, files.as_ref(), counter.as_ref()) })
                .await;
            let _ = this.update(cx, |this, cx| this.finish(intent, result, cx));
        }));
        self.start_job_ticker(cx);
    }

    pub(super) fn start_job_ticker(&mut self, cx: &mut Context<Self>) {
        self.jobs.job_ticker = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(JOB_TICK_MS))
                    .await;
                let running = this.update(cx, |this, cx| {
                    cx.notify();
                    this.jobs.job.is_some()
                });
                if !matches!(running, Ok(true)) {
                    break;
                }
            }
        }));
    }

    fn finish(
        &mut self,
        intent: OperationIntent,
        result: Result<FileOperation, OperationError>,
        cx: &mut Context<Self>,
    ) {
        self.jobs.operation_task = None;
        self.jobs.job = None;
        match result {
            Ok(done) => self.record(intent, done, cx),
            Err(error) => {
                match intent {
                    OperationIntent::Undo(operation) => self.history.push_undo(operation),
                    OperationIntent::Redo(operation) => self.history.push_redo(operation),
                    OperationIntent::Fresh
                    | OperationIntent::FreshThenRename
                    | OperationIntent::FreshInPlace => {}
                }
                self.show_error(error, cx);
                self.reload_everything(cx);
            }
        }
        if let Some((operation, intent)) = self.jobs.queue.pop_front() {
            self.execute(operation, intent, cx);
        }
    }

    fn record(&mut self, intent: OperationIntent, done: FileOperation, cx: &mut Context<Self>) {
        let rename = intent == OperationIntent::FreshThenRename;
        let (message, can_undo, landing) = match intent {
            OperationIntent::Fresh | OperationIntent::FreshThenRename => {
                self.history.record(done.clone());
                (Self::describe(&done), true, done.landing_path())
            }
            OperationIntent::FreshInPlace => {
                self.history.record(done.clone());
                (Self::describe(&done), true, None)
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

    pub fn cancel_job(&mut self) {
        if let Some(job) = &self.jobs.job {
            job.counter.cancel();
        }
    }

    pub fn job_card(&self) -> Option<JobCardModel> {
        let job = self
            .jobs
            .job
            .as_ref()
            .filter(|job| job.started.elapsed().as_millis() > JOB_CARD_DELAY_MS)?;
        let (done, total) = (job.counter.done(), job.counter.total());
        let fraction = (total > 0).then(|| (done as f32 / total as f32).min(1.0));
        let detail = if total > 0 {
            t!(
                "job.progress",
                done = file_size(done),
                total = file_size(total)
            )
            .to_string()
        } else {
            file_size(done)
        };
        Some(JobCardModel {
            title: t!(match job.counter.phase() {
                JobPhase::Working => job.title_key,
                JobPhase::Flushing => "job.flushing",
            })
            .to_string()
            .into(),
            item: SharedString::from(job.counter.item()),
            detail: detail.into(),
            fraction,
        })
    }
}
