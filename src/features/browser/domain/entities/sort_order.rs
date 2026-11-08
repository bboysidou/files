use crate::core::enums::browser::sort_key::SortKey;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SortOrder {
    pub key: SortKey,
    pub ascending: bool,
    pub hidden_last: bool,
}

impl SortOrder {
    pub fn by_name() -> Self {
        Self {
            key: SortKey::Name,
            ascending: true,
            hidden_last: false,
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
