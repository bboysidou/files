use std::rc::Rc;

use gpui::{Context, SharedString};
use rust_i18n::t;

use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::device_item::DeviceItem;
use crate::global_components::handlers::IndexHandler;

impl ShellView {
    pub(super) fn note_location(&mut self, cx: &mut Context<Self>) {
        let location = self.browser.read(cx).location(cx);
        if self.network.seen.as_ref() == Some(&location) {
            return;
        }
        self.network.seen = Some(location);
        self.refresh_network(cx);
    }

    pub(super) fn refresh_network(&mut self, cx: &mut Context<Self>) {
        let mounter = self.network.mounter.clone();
        self.network.task = Some(cx.spawn(async move |this, cx| {
            let mounts = cx
                .background_executor()
                .spawn(async move { mounter.mounts() })
                .await;
            let _ = this.update(cx, |this, cx| {
                if this.network.mounts != mounts {
                    this.network.mounts = mounts;
                    cx.notify();
                }
            });
        }));
    }

    pub(super) fn network_items(&self, location: &PlaceTarget) -> Vec<DeviceItem> {
        self.network
            .mounts
            .iter()
            .map(|mount| DeviceItem {
                label: SharedString::from(mount.label.clone()),
                mounted: true,
                active: matches!(location, PlaceTarget::Directory(path) if path.starts_with(&mount.path)),
            })
            .collect()
    }

    pub(super) fn open_network(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(mount) = self.network.mounts.get(index) else {
            return;
        };
        let target = PlaceTarget::Directory(mount.path.clone());
        self.browser
            .update(cx, |browser, cx| browser.navigate(target, cx));
    }

    fn unmount_network(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(mount) = self.network.mounts.get(index).cloned() else {
            return;
        };
        let inside = matches!(self.browser.read(cx).location(cx), PlaceTarget::Directory(path) if path.starts_with(&mount.path));
        if inside {
            let home = self.home_target.clone();
            self.browser
                .update(cx, |browser, cx| browser.navigate(home, cx));
        }
        let mounter = self.network.mounter.clone();
        self.network.task = Some(cx.spawn(async move |this, cx| {
            let path = mount.path.clone();
            let unmounted = cx
                .background_executor()
                .spawn(async move { mounter.unmount(&path) })
                .await;
            let _ = this.update(cx, |this, cx| {
                let message = match unmounted {
                    Ok(()) => t!("network.unmounted", label = mount.label).to_string(),
                    Err(reason) => t!(
                        "network.unmount_failed",
                        label = mount.label,
                        reason = reason
                    )
                    .to_string(),
                };
                this.report(message, cx);
                this.refresh_network(cx);
            });
        }));
    }

    pub(super) fn network_handlers(cx: &mut Context<Self>) -> (IndexHandler, IndexHandler) {
        let (open_view, unmount_view) = (cx.entity().downgrade(), cx.entity().downgrade());
        let on_open: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = open_view.update(cx, |this, cx| this.open_network(index, cx));
        });
        let on_unmount: IndexHandler = Rc::new(move |index, _, cx| {
            cx.stop_propagation();
            let _ = unmount_view.update(cx, |this, cx| this.unmount_network(index, cx));
        });
        (on_open, on_unmount)
    }
}
