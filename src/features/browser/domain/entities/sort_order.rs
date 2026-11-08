use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::enums::browser::sort_key::SortKey;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SortOrder {
    pub key: SortKey,
    pub ascending: bool,
    pub hidden_last: bool,
    pub seed: u32,
}

impl SortOrder {
    pub fn by_name() -> Self {
        Self {
            key: SortKey::Name,
            ascending: true,
            hidden_last: false,
            seed: 0,
        }
    }

    pub fn picked(self, key: SortKey) -> Self {
        Self {
            key,
            seed: if key == SortKey::Random {
                fresh_seed()
            } else {
                self.seed
            },
            ..self
        }
    }

    pub fn toggled_on(self, key: SortKey) -> Self {
        let ascending = key != self.key || !self.ascending;
        Self {
            key,
            ascending,
            ..self
        }
    }
}

fn fresh_seed() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.subsec_nanos())
}
