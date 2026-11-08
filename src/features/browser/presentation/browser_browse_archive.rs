use std::fs;
use std::path::PathBuf;

use gpui::{Context, Window};
use rust_i18n::t;

use crate::core::constants::preview::ARCHIVE_BROWSE_DIR;
use crate::core::helpers::files::archive_stem::archive_stem;
use crate::core::helpers::files::cache_name::cache_name;
use crate::core::helpers::files::is_archive::is_archive;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::places::domain::entities::place::PlaceTarget;

impl BrowserView {
    pub(super) fn browse_archive(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self
            .active_pane()
            .read(cx)
            .current_entry()
            .filter(is_archive)
        else {
            return self.show_toast(t!("archive.not_archive").to_string(), false, cx);
        };
        let Some(root) = dirs::cache_dir()
            .map(|dir| dir.join(ARCHIVE_BROWSE_DIR))
            .zip(cache_name(&entry.path))
            .map(|(dir, name)| dir.join(name))
        else {
            return;
        };
        let folder = root.join(archive_stem(&entry.name));
        if folder.is_dir() {
            return self.show_archive_copy((folder, entry.name), window, cx);
        }
        let (tool, archive, label) = (self.setup.archives.clone(), entry.path, entry.name);
        self.show_toast(
            t!("archive.extracting", name = label).to_string(),
            false,
            cx,
        );
        cx.spawn_in(window, async move |this, cx| {
            let destination = folder.clone();
            let outcome = cx
                .background_executor()
                .spawn(async move {
                    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
                    tool.extract(&archive, &destination)
                })
                .await;
            let _ = this.update_in(cx, |this, window, cx| match outcome {
                Ok(()) => this.show_archive_copy((folder, label), window, cx),
                Err(reason) => {
                    let message = t!("archive.failed", reason = reason).to_string();
                    this.show_toast(message, false, cx);
                }
            });
        })
        .detach();
    }

    fn show_archive_copy(
        &mut self,
        (folder, label): (PathBuf, String),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_tab((PlaceTarget::Directory(folder), None), window, cx);
        self.show_toast(t!("archive.browsing", name = label).to_string(), false, cx);
    }
}
