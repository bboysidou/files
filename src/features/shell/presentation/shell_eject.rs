use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use gpui::{AsyncApp, Context, WeakEntity};
use rust_i18n::t;

use crate::core::constants::devices::{EJECT_NOTE_MIN_BYTES, EJECT_POLL_MS, MOUNT_USERS_SEPARATOR};
use crate::core::constants::notifications::{
    NOTE_DONE_ICON, NOTE_FAILED_ICON, NOTE_NEVER_EXPIRES, NOTE_PERCENT_MAX,
};
use crate::core::dependency_injection::notifications::live_notes;
use crate::core::enums::notifications::note_urgency::NoteUrgency;
use crate::core::enums::operations::auth_mode::AuthMode;
use crate::core::helpers::files::pending_writeback::pending_writeback;
use crate::core::helpers::format::file_size::file_size;
use crate::features::devices::domain::entities::device::Device;
use crate::features::devices::domain::entities::device_error::DeviceError;
use crate::features::devices::domain::repositories::device_repository::DeviceRepository;
use crate::features::notifications::domain::entities::desktop_note::DesktopNote;
use crate::features::notifications::domain::usecases::live_notes::LiveNotes;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::shell_view::ShellView;

fn flush_note(label: &str, start: u64, pending: u64) -> DesktopNote {
    let written = start.saturating_sub(pending);
    let percent = (written * NOTE_PERCENT_MAX).checked_div(start).unwrap_or(0) as u8;
    let mut note = DesktopNote::new(
        t!("device.flushing", label = label).to_string(),
        t!("device.flush_left", size = file_size(pending)).to_string(),
    );
    note.progress = Some(percent);
    note.timeout_ms = NOTE_NEVER_EXPIRES;
    note.resident = true;
    note.sticky = true;
    note
}

fn result_note(device: &Device, outcome: Result<(), DeviceError>, users: &[String]) -> DesktopNote {
    match outcome {
        Ok(()) => {
            let mut note = DesktopNote::new(
                t!("device.removable", label = device.label).to_string(),
                String::new(),
            );
            note.icon = NOTE_DONE_ICON;
            note
        }
        Err(error) => {
            let body = if error == DeviceError::Busy && !users.is_empty() {
                t!("device.used_by", names = users.join(MOUNT_USERS_SEPARATOR)).to_string()
            } else {
                t!(error.label_key()).to_string()
            };
            let mut note = DesktopNote::new(
                t!("device.busy_title", label = device.label).to_string(),
                body,
            );
            note.icon = NOTE_FAILED_ICON;
            note.urgency = NoteUrgency::Critical;
            note
        }
    }
}

async fn flush_with_progress(
    repository: Arc<dyn DeviceRepository>,
    mount: PathBuf,
    note: (String, u64),
    cx: &mut AsyncApp,
) {
    let (label, slot) = note;
    let start = pending_writeback();
    let finished = Arc::new(AtomicBool::new(false));
    let flag = finished.clone();
    let flush = cx.background_executor().spawn(async move {
        let _ = repository.flush(&mount);
        flag.store(true, Ordering::Relaxed);
    });
    while !finished.load(Ordering::Relaxed) {
        if start > EJECT_NOTE_MIN_BYTES {
            live_notes().update(
                slot,
                flush_note(&label, start, pending_writeback().min(start)),
            );
        }
        cx.background_executor()
            .timer(Duration::from_millis(EJECT_POLL_MS))
            .await;
    }
    flush.await;
}

impl ShellView {
    pub(super) fn eject_device(&mut self, index: usize, cx: &mut Context<Self>) {
        if super::state::usb_format::FormatLease::busy() {
            self.report(t!("device.format.locked").to_string(), cx);
            return;
        }
        if let Some(device) = self.devices.get(index).cloned() {
            self.eject_with(device, AuthMode::Silent, cx);
        }
    }

    fn eject_with(&mut self, device: Device, mode: AuthMode, cx: &mut Context<Self>) {
        if let Some(mount) = device.mount.clone() {
            if self.refuse_busy(&device.label, &mount, cx) {
                return;
            }
            let inside = matches!(self.browser.read(cx).location(cx), PlaceTarget::Directory(path) if path.starts_with(&mount));
            if inside {
                let home = self.home_target.clone();
                self.browser
                    .update(cx, |browser, cx| browser.navigate(home, cx));
            }
        }
        let repository = self.device_repository.clone();
        self.device_task = Some(cx.spawn(async move |this, cx| {
            Self::eject_flow(repository, (device, mode), this, cx).await;
        }));
    }

    async fn eject_flow(
        repository: Arc<dyn DeviceRepository>,
        (device, mode): (Device, AuthMode),
        this: WeakEntity<Self>,
        cx: &mut AsyncApp,
    ) {
        let slot = LiveNotes::next_slot();
        if let Some(mount) = device.mount.clone() {
            flush_with_progress(repository.clone(), mount, (device.label.clone(), slot), cx).await;
        }
        let probe = device.clone();
        let ejected = cx
            .background_executor()
            .spawn(async move {
                let outcome = repository.eject(&probe, mode);
                let users = match (&outcome, &probe.mount) {
                    (Err(DeviceError::Busy), Some(mount)) => repository.users(mount),
                    _ => Vec::new(),
                };
                (outcome, users)
            })
            .await;
        let (outcome, users) = ejected;
        live_notes().finish(slot, Some(result_note(&device, outcome, &users)));
        let _ = this.update(cx, |this, cx| {
            if outcome == Err(DeviceError::NotAuthorized) && mode == AuthMode::Silent {
                let message = Self::failure(&device, DeviceError::NotAuthorized);
                let retry = this.retry_handler(device.clone(), Self::eject_with, cx);
                this.offer_auth(message, retry, cx);
                this.refresh_devices(cx);
                return;
            }
            let message = match outcome {
                Ok(()) => t!("device.ejected", label = device.label).to_string(),
                Err(DeviceError::Busy) if !users.is_empty() => t!(
                    "device.busy_users",
                    label = device.label,
                    names = users.join(MOUNT_USERS_SEPARATOR)
                )
                .to_string(),
                Err(error) => Self::failure(&device, error),
            };
            this.report(message, cx);
            this.refresh_devices(cx);
        });
    }
}
