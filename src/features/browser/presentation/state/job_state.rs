use std::sync::Arc;
use std::time::Instant;

use crate::features::operations::domain::entities::job_counter::JobCounter;

pub struct JobState {
    pub counter: Arc<JobCounter>,
    pub title_key: &'static str,
    pub started: Instant,
}
