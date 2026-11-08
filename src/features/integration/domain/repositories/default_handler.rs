pub trait DefaultHandler: Send + Sync {
    fn is_default(&self) -> bool;
    fn enable(&self) -> Result<String, String>;
    fn disable(&self) -> Result<String, String>;
}
