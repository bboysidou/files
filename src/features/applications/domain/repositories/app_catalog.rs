use std::path::Path;

use crate::features::applications::domain::entities::desktop_app::DesktopApp;

pub trait AppCatalog: Send + Sync {
    fn apps_for(&self, path: &Path) -> Vec<DesktopApp>;
}
