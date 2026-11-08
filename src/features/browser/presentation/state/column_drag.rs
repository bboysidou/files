use crate::core::enums::browser::sort_key::SortKey;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ColumnDrag {
    pub key: SortKey,
}
