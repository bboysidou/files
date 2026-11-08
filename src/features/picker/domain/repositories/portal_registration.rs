pub trait PortalRegistration: Send + Sync {
    fn register(&self) -> Result<String, String>;
}
