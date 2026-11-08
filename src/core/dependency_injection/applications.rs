use std::rc::Rc;
use std::sync::Arc;

use crate::features::applications::data::repositories::process_app_launcher::ProcessAppLauncher;
use crate::features::applications::data::repositories::xdg_app_catalog::XdgAppCatalog;
use crate::features::applications::domain::repositories::app_catalog::AppCatalog;
use crate::features::applications::domain::repositories::app_launcher::AppLauncher;

pub fn app_catalog() -> Arc<dyn AppCatalog> {
    Arc::new(XdgAppCatalog)
}

pub fn app_launcher() -> Rc<dyn AppLauncher> {
    Rc::new(ProcessAppLauncher)
}
