use crate::core::enums::operations::job_phase::JobPhase;

pub trait ProgressSink: Send + Sync {
    fn begin(&self, total_bytes: u64);
    fn advance(&self, bytes: u64);
    fn set_item(&self, name: &str);
    fn set_phase(&self, phase: JobPhase);
    fn is_cancelled(&self) -> bool;
}
