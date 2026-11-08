use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortKey {
    Name,
    Size,
    Kind,
    Modified,
    Extension,
    Created,
    Random,
}

impl SortKey {
    pub const ALL: [SortKey; 7] = [
        SortKey::Name,
        SortKey::Size,
        SortKey::Kind,
        SortKey::Modified,
        SortKey::Extension,
        SortKey::Created,
        SortKey::Random,
    ];

    pub fn label_key(self) -> &'static str {
        match self {
            SortKey::Name => "list.column.name",
            SortKey::Size => "list.column.size",
            SortKey::Kind => "list.column.kind",
            SortKey::Modified => "list.column.modified",
            SortKey::Extension => "list.column.extension",
            SortKey::Created => "list.column.created",
            SortKey::Random => "list.sort.random",
        }
    }
}
