use std::sync::{Arc, OnceLock};

use crate::features::admin::data::datasources::pkexec_session::PkexecSession;
use crate::features::admin::domain::repositories::admin_session::AdminSession;

static SESSION: OnceLock<Arc<PkexecSession>> = OnceLock::new();

pub fn pkexec_session() -> Arc<PkexecSession> {
    SESSION.get_or_init(Arc::default).clone()
}

pub fn admin_session() -> Arc<dyn AdminSession> {
    pkexec_session()
}
