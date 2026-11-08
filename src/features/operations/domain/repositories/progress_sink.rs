pub trait ProgressSink: Send + Sync {
    fn begin(&self, total_bytes: u64);
    fn advance(&self, bytes: u64);
    fn set_item(&self, name: &str);
    fn is_cancelled(&self) -> bool;
}
