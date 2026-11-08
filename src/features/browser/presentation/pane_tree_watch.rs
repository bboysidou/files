use std::path::PathBuf;
use std::time::Duration;

use futures::StreamExt;
use gpui::Context;

use crate::core::constants::files::WATCH_DEBOUNCE_MS;
use crate::features::browser::presentation::pane_view::PaneView;

impl PaneView {
    pub(super) fn sync_tree_watches(&mut self, cx: &mut Context<Self>) {
        let expanded = self.tree.expanded.clone();
        self.tree
            .watches
            .retain(|folder, _| expanded.contains(folder));
        for folder in expanded {
            if !self.tree.watches.contains_key(&folder) {
                self.watch_tree_folder(folder, cx);
            }
        }
    }

    fn watch_tree_folder(&mut self, folder: PathBuf, cx: &mut Context<Self>) {
        let Some(subscription) = self.setup.watcher.watch(&folder) else {
            return;
        };
        let (mut changes, guard) = (subscription.changes, subscription.guard);
        let watched = folder.clone();
        let task = cx.spawn(async move |this, cx| {
            let _guard = guard;
            while changes.next().await.is_some() {
                cx.background_executor()
                    .timer(Duration::from_millis(WATCH_DEBOUNCE_MS))
                    .await;
                while changes.try_recv().is_ok() {}
                let reload =
                    this.update(cx, |this, cx| this.reload_tree_folder(watched.clone(), cx));
                if reload.is_err() {
                    break;
                }
            }
        });
        self.tree.watches.insert(folder, task);
    }

    fn reload_tree_folder(&mut self, folder: PathBuf, cx: &mut Context<Self>) {
        let repository = self.setup.repository.clone();
        cx.spawn(async move |this, cx| {
            let target = folder.clone();
            let listed = cx
                .background_executor()
                .spawn(async move { repository.list(&target) })
                .await;
            let _ = this.update(cx, |this, cx| {
                if !this.tree.expanded.contains(&folder) {
                    return;
                }
                match listed {
                    Ok(children) => {
                        this.tree.children.insert(folder, children);
                    }
                    Err(_) => {
                        this.tree.expanded.retain(|path| !path.starts_with(&folder));
                        this.tree.children.remove(&folder);
                        this.sync_tree_watches(cx);
                    }
                }
                this.load(cx);
            });
        })
        .detach();
    }
}
