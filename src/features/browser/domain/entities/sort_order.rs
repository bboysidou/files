use crate::core::enums::browser::sort_key::SortKey;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SortOrder {
    pub key: SortKey,
    pub ascending: bool,
}

impl SortOrder {
    pub fn by_name() -> Self {
        Self {
            key: SortKey::Name,
            ascending: true,
        }
    }

    pub fn toggled_on(self, key: SortKey) -> Self {
        if key == self.key {
            Self {
                key,
                ascending: !self.ascending,
            }
        } else {
            Self {
                key,
                ascending: true,
            }
        }
    }
}
