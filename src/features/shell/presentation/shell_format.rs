use std::rc::Rc;

use crate::features::devices::domain::entities::format_request::FormatRequest;
use crate::features::devices::domain::usecases::volume_label::volume_label;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};
use gpui::{AppContext, Context, Focusable, Pixels, Point, Window};
use rust_i18n::t;

use crate::features::devices::domain::entities::device::Device;
use crate::features::devices::domain::entities::format_request::FormatError;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::usb_format::{
    FormatLease, FormatPhase, UsbFormat,
};
use crate::global_components::handlers::IndexPointHandler;

impl ShellView {
    pub(super) fn device_context_handler(&self, cx: &Context<Self>) -> IndexPointHandler {
        let view = cx.entity().downgrade();
        // Bind the handler to the rendered devices, rather than a later refreshed index.
        let devices = self.devices.clone();
        Rc::new(move |index, position, window, cx| {
            if let Some(device) = devices.get(index).cloned() {
                let _ = view.update(cx, |this, cx| {
                    this.open_device_menu((device, position), window, cx)
                });
            }
        })
    }

    fn open_device_menu(
        &mut self,
        (device, position): (Device, Point<Pixels>),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if FormatLease::busy() {
            self.report(t!("device.format.locked").to_string(), cx);
            return;
        }
        let label =
            cx.new(|cx| TextField::new(t!("device.format.label.placeholder").to_string(), cx));
        let events = cx.subscribe_in(&label, window, |this, _, event, window, cx| match event {
            TextFieldEvent::Changed => {
                if let Some(UsbFormat {
                    phase: FormatPhase::Choose { confirm, .. },
                    ..
                }) = this.usb_format.as_mut()
                {
                    *confirm = false;
                }
                cx.notify();
            }
            TextFieldEvent::Cancelled => this.close_format(window, cx),
            TextFieldEvent::Submitted => window.focus(&this.format_focus, cx),
        });
        self.usb_format = Some(UsbFormat {
            device,
            phase: FormatPhase::Menu(position),
            lease: None,
            label,
            _label_events: events,
        });
        window.focus(&self.format_focus, cx);
        cx.notify();
    }

    pub(super) fn close_format(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(
            self.usb_format.as_ref().map(|state| &state.phase),
            Some(FormatPhase::Running { .. })
        ) {
            return;
        }
        self.format_task = None;
        self.usb_format = None;
        window.focus(&self.browser.focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn load_format_options(&mut self, cx: &mut Context<Self>) {
        let Some(state) = self.usb_format.as_mut() else {
            return;
        };
        if !matches!(state.phase, FormatPhase::Menu(_)) {
            return;
        }
        let Some(lease) = FormatLease::acquire() else {
            self.report(t!("device.format.locked").to_string(), cx);
            return;
        };
        state.lease = Some(lease);
        state.phase = FormatPhase::Loading;
        let device = state.device.clone();
        let repository = self.device_repository.clone();
        self.format_task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { repository.format_options(&device) })
                .await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(options) => {
                        if let Some(state) = this.usb_format.as_mut() {
                            state.phase = FormatPhase::Choose {
                                options,
                                selected: 0,
                                confirm: false,
                            };
                        }
                    }
                    Err(error) => {
                        let message = t!(error.label_key()).to_string();
                        if let Some(state) = this.usb_format.as_mut() {
                            state.phase = FormatPhase::Finished(message.clone());
                            state.lease = None;
                        }
                        this.report(message, cx);
                    }
                }
                cx.notify();
            });
        }));
        cx.notify();
    }

    pub(super) fn select_format(&mut self, selected: usize, cx: &mut Context<Self>) {
        if let Some(UsbFormat {
            phase:
                FormatPhase::Choose {
                    options,
                    selected: current,
                    confirm,
                },
            ..
        }) = self.usb_format.as_mut()
            && selected < options.filesystems.len()
        {
            *current = selected;
            *confirm = false;
        }
        cx.notify();
    }

    pub(super) fn confirm_format(&mut self, cx: &mut Context<Self>) {
        let Some(state) = self.usb_format.as_mut() else {
            return;
        };
        let FormatPhase::Choose {
            options,
            selected,
            confirm,
        } = &mut state.phase
        else {
            return;
        };
        let Some(filesystem) = options.filesystems.get(*selected).copied() else {
            return;
        };
        let Ok(label) = volume_label(filesystem, state.label.read(cx).text()) else {
            return;
        };
        if !*confirm {
            *confirm = true;
            cx.notify();
            return;
        }
        let request = FormatRequest {
            target: options.target.clone(),
            filesystem,
            label,
        };
        self.run_usb_format(request, cx);
    }

    fn run_usb_format(&mut self, request: FormatRequest, cx: &mut Context<Self>) {
        let Some(state) = self.usb_format.as_ref() else {
            return;
        };
        let device = state.device.clone();
        let lease = state.lease.clone();
        if let Some(mount) = &device.mount {
            if self.refuse_busy(&device.label, mount, cx) {
                return;
            }
            self.leave_mount(mount, cx);
        }
        if let Some(state) = self.usb_format.as_mut() {
            state.phase = FormatPhase::Running { progress: None };
        }
        self.watch_format_progress(request.target.clone(), cx);
        let repository = self.device_repository.clone();
        let filesystem = request.filesystem;
        self.format_task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let _lease = lease;
                    repository.format(&request)
                })
                .await;
            let _ = this.update(cx, |this, cx| {
                this.format_progress_task = None;
                let message = match result {
                    Ok(()) => t!(
                        "device.format.done",
                        label = device.label,
                        filesystem = filesystem.label()
                    )
                    .to_string(),
                    Err(error) => format_failure(&device, error),
                };
                if let Some(state) = this.usb_format.as_mut() {
                    state.phase = FormatPhase::Finished(message.clone());
                    state.lease = None;
                }
                this.report(message, cx);
                this.refresh_devices(cx);
                cx.notify();
            });
        }));
        cx.notify();
    }
}

fn format_failure(device: &Device, error: FormatError) -> String {
    t!(
        "device.format.failed",
        label = device.label,
        reason = t!(error.label_key())
    )
    .to_string()
}
