use std::path::Path;

use crate::core::constants::applications::{OPEN_WITH_LIMIT, PLAIN_TEXT_MIME, TEXT_MIME_PREFIX};
use crate::features::applications::data::datasources::desktop_entry_scanner::scan_desktop_entries;
use crate::features::applications::data::datasources::mime_query::{default_app_id, file_mime};
use crate::features::applications::data::models::desktop_entry::DesktopEntry;
use crate::features::applications::domain::entities::desktop_app::DesktopApp;
use crate::features::applications::domain::repositories::app_catalog::AppCatalog;

pub struct XdgAppCatalog;

impl AppCatalog for XdgAppCatalog {
    fn apps_for(&self, path: &Path) -> Vec<DesktopApp> {
        let Some(mime) = file_mime(path) else {
            return Vec::new();
        };
        let default = default_app_id(&mime);
        let mut apps: Vec<DesktopApp> = scan_desktop_entries()
            .into_iter()
            .filter(|entry| handles(entry, &mime))
            .map(|entry| entry.app)
            .collect();
        apps.sort_by_cached_key(|app| {
            (
                default.as_deref() != Some(app.id.as_str()),
                app.name.to_lowercase(),
            )
        });
        apps.truncate(OPEN_WITH_LIMIT);
        apps
    }
}

fn handles(entry: &DesktopEntry, mime: &str) -> bool {
    let text = mime.starts_with(TEXT_MIME_PREFIX);
    entry
        .mime_types
        .iter()
        .any(|kind| kind == mime || (text && kind == PLAIN_TEXT_MIME))
}
