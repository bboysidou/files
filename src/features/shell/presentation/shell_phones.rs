use std::path::Path;
use std::rc::Rc;

use gpui::{Context, SharedString};
use rust_i18n::t;

use crate::core::constants::phones::PHONE_REFRESH_DEBOUNCE_MS;
use crate::features::network::domain::entities::network_mount_error::NetworkMountError;
use crate::features::phones::domain::entities::phone::Phone;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::device_item::DeviceItem;
use crate::global_components::handlers::IndexHandler;

impl ShellView {
    pub(super) fn watch_phones(&mut self, cx: &mut Context<Self>) {
        self.refresh_phones(cx);
        let Some(subscription) = self.phones.watcher.watch() else {
            return;
        };
        self.phones.watch = Some(Self::debounced_refresh(
            subscription,
            PHONE_REFRESH_DEBOUNCE_MS,
            Self::refresh_phones,
            cx,
        ));
    }

    pub(super) fn refresh_phones(&mut self, cx: &mut Context<Self>) {
        let repository = self.phones.repository.clone();
        self.phones.task = Some(cx.spawn(async move |this, cx| {
            let phones = cx
                .background_executor()
                .spawn(async move { repository.phones() })
                .await;
            let _ = this.update(cx, |this, cx| {
                if this.phones.phones != phones {
                    this.phones.phones = phones;
                    cx.notify();
                }
            });
        }));
    }

    pub(super) fn phone_items(&self, location: &PlaceTarget) -> Vec<DeviceItem> {
        self.phones
            .phones
            .iter()
            .map(|phone| DeviceItem {
                label: SharedString::from(phone.name.clone()),
                mounted: phone.mount.is_some(),
                active: matches!((location, &phone.mount), (PlaceTarget::Directory(path), Some(mount)) if path.starts_with(mount)),
                saved: false,
                protocol: None,
            })
            .collect()
    }

    pub(super) fn open_phone(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(phone) = self.phones.phones.get(index).cloned() else {
            return;
        };
        if let Some(mount) = phone.mount {
            return self.browser.update(cx, |browser, cx| {
                browser.navigate(PlaceTarget::Directory(mount), cx)
            });
        }
        let mounter = self.network.mounter.clone();
        self.phones.mount_task = Some(cx.spawn(async move |this, cx| {
            let uri = phone.uri.clone();
            let mounted = cx
                .background_executor()
                .spawn(async move { mounter.mount(&uri, None) })
                .await;
            let _ = this.update(cx, |this, cx| match mounted {
                Ok(path) => {
                    this.browser.update(cx, |browser, cx| {
                        browser.navigate(PlaceTarget::Directory(path), cx)
                    });
                    this.refresh_phones(cx);
                }
                Err(error) => this.report(phone_failure(&phone, error), cx),
            });
        }));
    }

    pub(super) fn eject_phone(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(phone) = self.phones.phones.get(index).cloned() else {
            return;
        };
        let Some(mount) = phone.mount else {
            return;
        };
        if self.refuse_busy(&phone.name, &mount, cx) {
            return;
        }
        self.leave_mount(&mount, cx);
        let mounter = self.network.mounter.clone();
        let target = mount.clone();
        self.phones.mount_task = Some(cx.spawn(async move |this, cx| {
            let unmounted = cx
                .background_executor()
                .spawn(async move { mounter.unmount(&target) })
                .await;
            let _ = this.update(cx, |this, cx| {
                let label = phone.name;
                let message = match unmounted {
                    Ok(()) => t!("phone.ejected", label = label).to_string(),
                    Err(reason) => {
                        t!("phone.eject_failed", label = label, reason = reason).to_string()
                    }
                };
                this.report(message, cx);
                this.refresh_phones(cx);
            });
        }));
    }

    pub(super) fn leave_mount(&mut self, mount: &Path, cx: &mut Context<Self>) {
        let inside = matches!(self.browser.read(cx).location(cx), PlaceTarget::Directory(path) if path.starts_with(mount));
        if inside {
            let home = self.home_target.clone();
            self.browser
                .update(cx, |browser, cx| browser.navigate(home, cx));
        }
    }

    pub(super) fn phone_handlers(cx: &mut Context<Self>) -> (IndexHandler, IndexHandler) {
        let (open_view, eject_view) = (cx.entity().downgrade(), cx.entity().downgrade());
        let on_open: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = open_view.update(cx, |this, cx| this.open_phone(index, cx));
        });
        let on_eject: IndexHandler = Rc::new(move |index, _, cx| {
            cx.stop_propagation();
            let _ = eject_view.update(cx, |this, cx| this.eject_phone(index, cx));
        });
        (on_open, on_eject)
    }
}

fn phone_failure(phone: &Phone, error: NetworkMountError) -> String {
    let reason = match error {
        NetworkMountError::TimedOut => t!("phone.timed_out").to_string(),
        NetworkMountError::Failed(reason) => reason,
        NetworkMountError::NeedsCredentials | NetworkMountError::Rejected => {
            t!("phone.refused").to_string()
        }
    };
    t!("phone.failed", label = phone.name, reason = reason).to_string()
}
