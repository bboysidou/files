use crate::features::applications::domain::entities::desktop_app::DesktopApp;

pub struct DesktopEntry {
    pub app: DesktopApp,
    pub mime_types: Vec<String>,
}
