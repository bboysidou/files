use std::path::Path;
use std::rc::Rc;

use gpui::{Context, SharedString};
use rust_i18n::t;

use crate::core::constants::devices::{DEVICE_EVENTS_DIR, DEVICE_REFRESH_DEBOUNCE_MS};
use crate::core::enums::operations::auth_mode::AuthMode;
use crate::features::browser::presentation::state::toast_model::ToastAction;
use crate::features::devices::domain::entities::device::Device;
use crate::features::devices::domain::entities::device_error::DeviceError;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::device_item::DeviceItem;
use crate::global_components::handlers::{ClickHandler, IndexHandler};

impl ShellView {
    pub(super) fn refresh_devices(&mut self, cx: &mut Context<Self>) {
        let repository = self.device_repository.clone();
        self.device_list_task = Some(cx.spawn(async move |this, cx| {
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
        self.device_watch = Some(Self::debounced_refresh(
            subscription,
            DEVICE_REFRESH_DEBOUNCE_MS,
            Self::refresh_devices,
            cx,
        ));
    }

    pub(super) fn device_items(&self, location: &PlaceTarget) -> Vec<DeviceItem> {
        self.devices
            .iter()
            .map(|device| DeviceItem {
                label: SharedString::from(device.label.clone()),
                mounted: device.mount.is_some(),
                saved: false,
                protocol: None,
                active: device
                    .mount
                    .as_ref()
                    .is_some_and(|mount| *location == PlaceTarget::Directory(mount.clone())),
            })
            .collect()
    }

    pub(super) fn report(&mut self, message: String, cx: &mut Context<Self>) {
        self.browser
            .update(cx, |browser, cx| browser.notify_message(message, cx));
    }

    pub(super) fn failure(device: &Device, error: DeviceError) -> String {
        t!(
            "device.failed",
            label = device.label,
            reason = t!(error.label_key())
        )
        .to_string()
    }

    pub(super) fn open_device(&mut self, index: usize, cx: &mut Context<Self>) {
        if super::state::usb_format::FormatLease::busy() {
            self.report(t!("device.format.locked").to_string(), cx);
            return;
        }
        if let Some(device) = self.devices.get(index).cloned() {
            self.mount_device(device, AuthMode::Silent, cx);
        }
    }

    fn mount_device(&mut self, device: Device, mode: AuthMode, cx: &mut Context<Self>) {
        let repository = self.device_repository.clone();
        self.device_task = Some(cx.spawn(async move |this, cx| {
            let probe = device.clone();
            let mounted = cx
                .background_executor()
                .spawn(async move { repository.mount(&probe, mode) })
                .await;
            let _ = this.update(cx, |this, cx| match mounted {
                Ok(path) => {
                    this.browser.update(cx, |browser, cx| {
                        browser.navigate(PlaceTarget::Directory(path), cx)
                    });
                    this.refresh_devices(cx);
                }
                Err(DeviceError::NotAuthorized) if mode == AuthMode::Silent => {
                    let message = Self::failure(&device, DeviceError::NotAuthorized);
                    let retry = this.retry_handler(device, Self::mount_device, cx);
                    this.offer_auth(message, retry, cx);
                }
                Err(error) => this.report(Self::failure(&device, error), cx),
            });
        }));
    }

    pub(super) fn retry_handler(
        &self,
        device: Device,
        run: fn(&mut Self, Device, AuthMode, &mut Context<Self>),
        cx: &mut Context<Self>,
    ) -> ClickHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |_, cx| {
            let device = device.clone();
            let _ = view.update(cx, |this, cx| run(this, device, AuthMode::Interactive, cx));
        })
    }

    pub(super) fn offer_auth(
        &mut self,
        message: String,
        retry: ClickHandler,
        cx: &mut Context<Self>,
    ) {
        let label = SharedString::from(t!("auth.authenticate").to_string());
        self.browser.update(cx, |browser, cx| {
            browser.show_toast_action(message, Some(ToastAction::Run(label, retry)), cx)
        });
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
