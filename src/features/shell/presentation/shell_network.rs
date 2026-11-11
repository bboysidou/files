use std::path::PathBuf;
use std::rc::Rc;

use gpui::{App, Context, SharedString};
use rust_i18n::t;

use crate::core::constants::network::NETWORK_REFRESH_DEBOUNCE_MS;
use crate::features::browser::presentation::state::pins::Pins;
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

    pub(super) fn watch_network(&mut self, cx: &mut Context<Self>) {
        let Some(subscription) = self.network.watcher.watch() else {
            return;
        };
        self.network.watch = Some(Self::debounced_refresh(
            subscription,
            NETWORK_REFRESH_DEBOUNCE_MS,
            Self::refresh_network,
            cx,
        ));
    }

    pub(super) fn refresh_network(&mut self, cx: &mut Context<Self>) {
        let (mounter, ssh) = (self.network.mounter.clone(), self.network.ssh.clone());
        self.network.task = Some(cx.spawn(async move |this, cx| {
            let (mounts, hosts) = cx
                .background_executor()
                .spawn(async move { (mounter.mounts(), ssh.hosts()) })
                .await;
            let _ = this.update(cx, |this, cx| {
                if this.network.mounts != mounts || this.network.ssh_hosts != hosts {
                    this.network.mounts = mounts;
                    this.network.ssh_hosts = hosts;
                    cx.notify();
                }
            });
        }));
    }

    pub(super) fn hide_server(&mut self, uri: &str, cx: &mut Context<Self>) {
        cx.global_mut::<Pins>().0.toggle_server(uri);
        self.persist(cx);
        cx.refresh_windows();
        cx.notify();
    }

    pub(super) fn network_items(&self, location: &PlaceTarget, cx: &App) -> Vec<DeviceItem> {
        self.network_entries(cx)
            .into_iter()
            .map(|entry| DeviceItem {
                active: matches!((location, &entry.mount), (PlaceTarget::Directory(path), Some(mount)) if path.starts_with(mount)),
                label: SharedString::from(entry.label),
                mounted: entry.mount.is_some(),
                saved: entry.uri.is_some(),
                protocol: entry.protocol,
            })
            .collect()
    }

    pub(super) fn open_network(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.network_entries(cx).into_iter().nth(index) else {
            return;
        };
        match (entry.mount, entry.uri) {
            (Some(mount), _) => {
                let target = PlaceTarget::Directory(mount);
                self.browser
                    .update(cx, |browser, cx| browser.navigate(target, cx));
            }
            (None, Some(uri)) => self
                .browser
                .update(cx, |browser, cx| browser.connect_server(uri, cx)),
            (None, None) => {}
        }
    }

    pub(super) fn network_trailing(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.network_entries(cx).into_iter().nth(index) else {
            return;
        };
        match (entry.mount, entry.uri) {
            (Some(mount), _) => self.unmount_network(entry.label, mount, cx),
            (None, Some(uri)) => self.hide_server(&uri, cx),
            (None, None) => {}
        }
    }

    pub(super) fn unmount_network(
        &mut self,
        label: String,
        mount: PathBuf,
        cx: &mut Context<Self>,
    ) {
        if self.refuse_busy(&label, &mount, cx) {
            return;
        }
        self.leave_mount(&mount, cx);
        let mounter = self.network.mounter.clone();
        self.network.unmount_task = Some(cx.spawn(async move |this, cx| {
            let unmounted = cx
                .background_executor()
                .spawn(async move { mounter.unmount(&mount) })
                .await;
            let _ = this.update(cx, |this, cx| {
                let message = match unmounted {
                    Ok(()) => t!("network.unmounted", label = label).to_string(),
                    Err(reason) => {
                        t!("network.unmount_failed", label = label, reason = reason).to_string()
                    }
                };
                this.report(message, cx);
                this.refresh_network(cx);
            });
        }));
    }

    pub(super) fn network_edit_handler(cx: &mut Context<Self>) -> IndexHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |index, window, cx| {
            cx.stop_propagation();
            let _ = view.update(cx, |this, cx| this.edit_server(index, window, cx));
        })
    }

    pub(super) fn network_handlers(cx: &mut Context<Self>) -> (IndexHandler, IndexHandler) {
        let (open_view, trailing_view) = (cx.entity().downgrade(), cx.entity().downgrade());
        let on_open: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = open_view.update(cx, |this, cx| this.open_network(index, cx));
        });
        let on_trailing: IndexHandler = Rc::new(move |index, _, cx| {
            cx.stop_propagation();
            let _ = trailing_view.update(cx, |this, cx| this.network_trailing(index, cx));
        });
        (on_open, on_trailing)
    }
}
