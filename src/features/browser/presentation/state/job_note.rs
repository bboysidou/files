use std::collections::VecDeque;
use std::time::Instant;

use crate::core::dependency_injection::notifications::live_notes;
use crate::features::notifications::domain::usecases::live_notes::LiveNotes;

pub struct JobNote {
    pub slot: u64,
    pub shown: bool,
    pub last_percent: u8,
    pub last_at: Option<Instant>,
    pub samples: VecDeque<(Instant, u64)>,
}

impl Default for JobNote {
    fn default() -> Self {
        Self {
            slot: LiveNotes::next_slot(),
            shown: false,
            last_percent: 0,
            last_at: None,
            samples: VecDeque::new(),
        }
    }
}

impl Drop for JobNote {
    fn drop(&mut self) {
        if self.shown {
            live_notes().finish(self.slot, None);
        }
    }
}
