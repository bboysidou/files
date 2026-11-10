use std::process;
use std::time::Instant;

use rust_i18n::t;

use crate::core::constants::notifications::{
    NOTE_JOB_TAG_PREFIX, NOTE_NEVER_EXPIRES, NOTE_PART_SEPARATOR, NOTE_PERCENT_MAX,
    NOTE_SPEED_WINDOW_MS, NOTE_UPDATE_MIN_MS, NOTE_UPDATE_MIN_STEP,
};
use crate::core::constants::operations::JOB_CARD_DELAY_MS;
use crate::core::dependency_injection::notifications::live_notes;
use crate::core::enums::notifications::note_urgency::NoteUrgency;
use crate::core::enums::operations::job_phase::JobPhase;
use crate::core::helpers::format::duration_text::duration_text;
use crate::core::helpers::format::file_size::file_size;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::job_state::JobState;
use crate::features::browser::presentation::state::note_outcome::NoteOutcome;
use crate::features::notifications::domain::entities::desktop_note::DesktopNote;

const MS_PER_SECOND: u128 = 1000;

fn tag(slot: u64) -> String {
    format!("{NOTE_JOB_TAG_PREFIX}{}-{slot}", process::id())
}

fn verb(job: &JobState) -> String {
    t!(format!("note.{}", job.title_key)).to_string()
}

fn rate(job: &mut JobState, now: Instant, done: u64) -> u64 {
    let samples = &mut job.note.samples;
    samples.push_back((now, done));
    while samples
        .front()
        .is_some_and(|(at, _)| now.duration_since(*at).as_millis() > NOTE_SPEED_WINDOW_MS)
    {
        samples.pop_front();
    }
    let Some((first_at, first_done)) = samples.front().copied() else {
        return 0;
    };
    let elapsed = now.duration_since(first_at).as_millis();
    if elapsed == 0 {
        return 0;
    }
    (u128::from(done.saturating_sub(first_done)) * MS_PER_SECOND / elapsed) as u64
}

fn body(job: &JobState, done: u64, total: u64, speed: u64) -> String {
    let item = job.counter.item();
    let mut parts: Vec<String> = Vec::new();
    if !item.is_empty() {
        parts.push(item);
    }
    parts.push(if total > 0 {
        t!(
            "job.progress",
            done = file_size(done),
            total = file_size(total)
        )
        .to_string()
    } else {
        file_size(done)
    });
    if speed > 0 {
        parts.push(t!("note.job_speed", speed = file_size(speed)).to_string());
        if total > done {
            let left = u128::from(total - done) * MS_PER_SECOND / u128::from(speed);
            parts.push(t!("note.job_left", time = duration_text(left as u64)).to_string());
        }
    }
    parts.join(NOTE_PART_SEPARATOR)
}

impl BrowserView {
    pub(super) fn note_tick(&mut self) {
        let Some(job) = self.jobs.job.as_mut() else {
            return;
        };
        if job.started.elapsed().as_millis() <= JOB_CARD_DELAY_MS {
            return;
        }
        let now = Instant::now();
        let (done, total) = (job.counter.done(), job.counter.total());
        let percent = (done.min(total) * NOTE_PERCENT_MAX)
            .checked_div(total)
            .unwrap_or(0) as u8;
        let speed = rate(job, now, done);
        let recent = job
            .note
            .last_at
            .is_some_and(|at| now.duration_since(at).as_millis() < NOTE_UPDATE_MIN_MS);
        let moved = percent.abs_diff(job.note.last_percent) >= NOTE_UPDATE_MIN_STEP;
        if job.note.shown && (recent || !moved) {
            return;
        }
        let summary = match job.counter.phase() {
            JobPhase::Flushing => t!("note.job_writing").to_string(),
            JobPhase::Working => verb(job),
        };
        let mut note = DesktopNote::new(summary, body(job, done, total, speed));
        note.progress = Some(percent);
        note.tag = Some(tag(job.note.slot));
        note.timeout_ms = NOTE_NEVER_EXPIRES;
        note.resident = true;
        note.sticky = true;
        live_notes().update(job.note.slot, note);
        job.note.shown = true;
        job.note.last_percent = percent;
        job.note.last_at = Some(now);
    }

    pub(super) fn note_finish(job: Option<JobState>, outcome: NoteOutcome) {
        let Some(mut job) = job else {
            return;
        };
        if !job.note.shown {
            return;
        }
        let what = verb(&job);
        let (summary, body, urgency) = match outcome {
            NoteOutcome::Done(message) => (message, job.counter.item(), NoteUrgency::Normal),
            NoteOutcome::Failed(reason) => (
                t!("note.job_failed", what = what).to_string(),
                reason,
                NoteUrgency::Critical,
            ),
            NoteOutcome::Cancelled => (
                t!("note.job_cancelled", what = what).to_string(),
                job.counter.item(),
                NoteUrgency::Low,
            ),
        };
        let mut note = DesktopNote::new(summary, body);
        note.urgency = urgency;
        note.tag = Some(tag(job.note.slot));
        live_notes().finish(job.note.slot, Some(note));
        job.note.shown = false;
    }
}
