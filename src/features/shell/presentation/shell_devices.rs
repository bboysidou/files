use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use futures::StreamExt;
use gpui::{Context, SharedString};
use rust_i18n::t;

use crate::core::constants::devices::{DEVICE_EVENTS_DIR, DEVICE_REFRESH_DEBOUNCE_MS};
use crate::features::devices::domain::entities::device::Device;
use crate::features::devices::domain::entities::device_error::DeviceError;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::device_item::DeviceItem;
use crate::global_components::handlers::IndexHandler;

impl ShellView {
    pub(super) fn refresh_devices(&mut self, cx: &mut Context<Self>) {
        let repository = self.device_repository.clone();
        self.device_task = Some(cx.spawn(async move |this, cx| {
            let devices = cx
                .background_executor()
                .spawn(async move { repository.list() })
                .await;
            let _ = this.update(cx, |this, cx| {
                this.devices = devices;
                cx.notify();
            });
        }));
    }

    pub(super) fn watch_devices(&mut self, cx: &mut Context<Self>) {
        let Some(subscription) = self.directory_watcher.watch(Path::new(DEVICE_EVENTS_DIR)) else {
            return;
        };
        let (mut changes, guard) = (subscription.changes, subscription.guard);
        self.device_watch = Some(cx.spawn(async move |this, cx| {
            let _guard = guard;
            while changes.next().await.is_some() {
                cx.background_executor()
                    .timer(Duration::from_millis(DEVICE_REFRESH_DEBOUNCE_MS))
                    .await;
                while changes.try_recv().is_ok() {}
                if this
                    .update(cx, |this, cx| this.refresh_devices(cx))
                    .is_err()
                {
                    break;
                }
            }
        }));
    }

    pub(super) fn device_items(&self, location: &PlaceTarget) -> Vec<DeviceItem> {
        self.devices
            .iter()
            .map(|device| DeviceItem {
                label: SharedString::from(device.label.clone()),
                mounted: device.mount.is_some(),
                active: device
                    .mount
                    .as_ref()
                    .is_some_and(|mount| *location == PlaceTarget::Directory(mount.clone())),
            })
            .collect()
    }

    fn report(&mut self, message: String, cx: &mut Context<Self>) {
        self.browser
            .update(cx, |browser, cx| browser.notify_message(message, cx));
    }

    fn failure(device: &Device, error: DeviceError) -> String {
        t!(
            "device.failed",
            label = device.label,
            reason = t!(error.label_key())
        )
        .to_string()
    }

    fn open_device(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(device) = self.devices.get(index).cloned() else {
            return;
        };
        let repository = self.device_repository.clone();
        self.device_task = Some(cx.spawn(async move |this, cx| {
            let probe = device.clone();
            let mounted = cx
                .background_executor()
                .spawn(async move { repository.mount(&probe) })
                .await;
            let _ = this.update(cx, |this, cx| match mounted {
                Ok(path) => {
                    this.browser.update(cx, |browser, cx| {
                        browser.navigate(PlaceTarget::Directory(path), cx)
                    });
                    this.refresh_devices(cx);
                }
                Err(error) => this.report(Self::failure(&device, error), cx),
            });
        }));
    }

    fn eject_device(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(device) = self.devices.get(index).cloned() else {
            return;
        };
        if let Some(mount) = device.mount.clone() {
            let inside = matches!(self.browser.read(cx).location(cx), PlaceTarget::Directory(path) if path.starts_with(&mount));
            if inside {
                let home = self.home_target.clone();
                self.browser
                    .update(cx, |browser, cx| browser.navigate(home, cx));
            }
        }
        let repository = self.device_repository.clone();
        self.device_task = Some(cx.spawn(async move |this, cx| {
            let probe = device.clone();
            let ejected = cx
                .background_executor()
                .spawn(async move { repository.eject(&probe) })
                .await;
            let _ = this.update(cx, |this, cx| {
                let message = match ejected {
                    Ok(()) => t!("device.ejected", label = device.label).to_string(),
                    Err(error) => Self::failure(&device, error),
                };
                this.report(message, cx);
                this.refresh_devices(cx);
            });
        }));
    }

    pub(super) fn device_handlers(cx: &mut Context<Self>) -> (IndexHandler, IndexHandler) {
        let (open_view, eject_view) = (cx.entity().downgrade(), cx.entity().downgrade());
        let on_device: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = open_view.update(cx, |this, cx| this.open_device(index, cx));
        });
        let on_eject: IndexHandler = Rc::new(move |index, _, cx| {
            cx.stop_propagation();
            let _ = eject_view.update(cx, |this, cx| this.eject_device(index, cx));
        });
        (on_device, on_eject)
    }
}
