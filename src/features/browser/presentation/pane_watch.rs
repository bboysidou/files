use std::path::PathBuf;
use std::time::Duration;

use futures::StreamExt;
use gpui::Context;

use crate::core::constants::files::WATCH_DEBOUNCE_MS;
use crate::features::browser::presentation::pane_view::PaneView;

impl PaneView {
    pub(super) fn watch(&mut self, directory: Option<PathBuf>, cx: &mut Context<Self>) {
        if directory == self.watching.dir {
            return;
        }
        self.watching.dir = directory.clone();
        self.watching.task = None;
        let Some(subscription) =
            directory.and_then(|directory| self.setup.watcher.watch(&directory))
        else {
            return;
        };
        let mut changes = subscription.changes;
        let guard = subscription.guard;
        self.watching.task = Some(cx.spawn(async move |this, cx| {
            let _guard = guard;
            while changes.next().await.is_some() {
                cx.background_executor()
                    .timer(Duration::from_millis(WATCH_DEBOUNCE_MS))
                    .await;
                while changes.try_recv().is_ok() {}
                if this.update(cx, |this, cx| this.load(cx)).is_err() {
                    break;
                }
            }
        }));
    }
}
