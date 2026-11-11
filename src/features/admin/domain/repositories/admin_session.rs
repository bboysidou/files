use crate::features::admin::domain::entities::admin_error::AdminError;

pub trait AdminSession: Send + Sync {
    fn is_active(&self) -> bool;
    fn start(&self) -> Result<(), AdminError>;
    fn end(&self);
}
