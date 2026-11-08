use gpui::Context;
use rust_i18n::t;

use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::features::browser::presentation::browser_view::BrowserView;

impl BrowserView {
    pub(super) fn cancel_clipboard(&mut self, cx: &mut Context<Self>) {
        let Some(pending) = self.clipboard.take() else {
            return;
        };
        self.sync_cut(cx);
        let system = self.setup.system_clipboard.clone();
        let ours = pending.paths.clone();
        cx.background_executor()
            .spawn(async move {
                if system.read().is_some_and(|current| current.paths == ours) {
                    system.clear();
                }
            })
            .detach();
        let message = match pending.mode {
            ClipboardMode::Cut => t!("op.move_cancelled"),
            ClipboardMode::Copy => t!("op.copy_cancelled"),
        };
        self.show_toast(message.to_string(), false, cx);
    }
}
