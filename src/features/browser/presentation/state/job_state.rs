use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use crate::features::browser::presentation::state::job_note::JobNote;
use crate::features::operations::domain::entities::job_counter::JobCounter;

pub struct JobState {
    pub counter: Arc<JobCounter>,
    pub title_key: &'static str,
    pub started: Instant,
    pub note: JobNote,
    pub paths: Vec<PathBuf>,
}
