use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::global_components::text_field::text_field_view::TextField;
use gpui::{Entity, Pixels, Point, Subscription};

use crate::features::devices::domain::entities::device::Device;
use crate::features::devices::domain::entities::format_request::FormatOptions;

static FORMATTING: AtomicBool = AtomicBool::new(false);

pub struct FormatLease;

impl FormatLease {
    pub fn acquire() -> Option<Arc<Self>> {
        FORMATTING
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .ok()
            .map(|_| Arc::new(Self))
    }

    pub fn busy() -> bool {
        FORMATTING.load(Ordering::SeqCst)
    }
}

impl Drop for FormatLease {
    fn drop(&mut self) {
        FORMATTING.store(false, Ordering::SeqCst);
    }
}

pub enum FormatPhase {
    Menu(Point<Pixels>),
    Loading,
    Choose {
        options: FormatOptions,
        selected: usize,
        confirm: bool,
    },
    Running {
        progress: Option<f32>,
    },
    Finished(String),
}

pub struct UsbFormat {
    pub device: Device,
    pub phase: FormatPhase,
    pub lease: Option<Arc<FormatLease>>,
    pub label: Entity<TextField>,
    pub _label_events: Subscription,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shared_lease_prevents_overlapping_formats_until_the_worker_finishes() {
        let dialog = FormatLease::acquire().expect("first dialog owns the lease");
        let worker = dialog.clone();
        assert!(FormatLease::busy());
        assert!(FormatLease::acquire().is_none());
        drop(dialog);
        assert!(FormatLease::busy());
        drop(worker);
        assert!(!FormatLease::busy());
        assert!(FormatLease::acquire().is_some());
    }
}
