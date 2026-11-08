use serde::{Deserialize, Serialize};

use crate::core::enums::browser::sort_key::SortKey;
use crate::core::theme::metrics::{
    LIST_COLUMN_MAX_WIDTH, LIST_COLUMN_MIN_WIDTH, LIST_KIND_WIDTH, LIST_MODIFIED_WIDTH,
    LIST_SIZE_WIDTH,
};

const HIDDEN: u16 = 0;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ListColumns {
    pub size: u16,
    pub kind: u16,
    pub modified: u16,
    pub created: u16,
}

impl Default for ListColumns {
    fn default() -> Self {
        Self {
            size: LIST_SIZE_WIDTH as u16,
            kind: LIST_KIND_WIDTH as u16,
            modified: LIST_MODIFIED_WIDTH as u16,
            created: HIDDEN,
        }
    }
}

impl ListColumns {
    const DROP_ORDER: [SortKey; 4] = [
        SortKey::Created,
        SortKey::Kind,
        SortKey::Modified,
        SortKey::Size,
    ];

    pub const KEYS: [SortKey; 4] = [
        SortKey::Size,
        SortKey::Kind,
        SortKey::Modified,
        SortKey::Created,
    ];

    pub fn width_of(&self, key: SortKey) -> Option<u16> {
        let width = match key {
            SortKey::Size => self.size,
            SortKey::Kind => self.kind,
            SortKey::Modified => self.modified,
            SortKey::Created => self.created,
            _ => HIDDEN,
        };
        (width != HIDDEN).then_some(width)
    }

    pub fn with_width(mut self, key: SortKey, width: Option<u16>) -> Self {
        let width = width.map_or(HIDDEN, |width| {
            width.clamp(LIST_COLUMN_MIN_WIDTH, LIST_COLUMN_MAX_WIDTH)
        });
        match key {
            SortKey::Size => self.size = width,
            SortKey::Kind => self.kind = width,
            SortKey::Modified => self.modified = width,
            SortKey::Created => self.created = width,
            _ => {}
        }
        self
    }

    pub fn visible(&self, compact: bool) -> Vec<(SortKey, u16)> {
        Self::KEYS
            .iter()
            .filter(|key| !(compact && matches!(key, SortKey::Kind | SortKey::Created)))
            .filter_map(|key| Some((*key, self.width_of(*key)?)))
            .collect()
    }

    pub fn fitting(&self, compact: bool, available: f32, reserved: f32) -> Vec<(SortKey, u16)> {
        if available <= 0.0 {
            return Vec::new();
        }
        let mut shown = self.visible(compact);
        let total = |shown: &[(SortKey, u16)]| {
            reserved
                + shown
                    .iter()
                    .map(|(_, width)| f32::from(*width))
                    .sum::<f32>()
        };
        for key in Self::DROP_ORDER {
            if total(&shown) <= available {
                break;
            }
            shown.retain(|(shown_key, _)| *shown_key != key);
        }
        shown
    }
}
