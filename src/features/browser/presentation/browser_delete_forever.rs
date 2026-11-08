use std::path::PathBuf;

use gpui::Context;
use rust_i18n::t;

use crate::core::helpers::format::item_count::item_count;
use crate::features::browser::presentation::browser_view::BrowserView;

impl BrowserView {
    pub(super) fn delete_forever(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        let operations = self.setup.operations.clone();
        let message = t!("op.purged", items = item_count(paths.len())).to_string();
        self.trash_task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { operations.delete(&paths) })
                .await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(()) => this.show_toast(message, false, cx),
                    Err(error) => this.show_error(error, cx),
                }
                this.reload_everything(cx);
            });
        }));
    }
}
