use std::cmp::Ordering;

use rust_i18n::t;

use crate::core::enums::browser::sort_key::SortKey;
use crate::core::helpers::text::natural_compare::natural_compare;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::sort_order::SortOrder;

pub fn arrange_entries(
    entries: &[FileEntry],
    order: SortOrder,
    show_hidden: bool,
) -> Vec<FileEntry> {
    let mut visible: Vec<FileEntry> = entries
        .iter()
        .filter(|entry| show_hidden || !entry.hidden)
        .cloned()
        .collect();
    visible.sort_by(|left, right| {
        right.is_dir.cmp(&left.is_dir).then_with(|| {
            let order_by_key = compare_by(left, right, order.key);
            if order.ascending {
                order_by_key
            } else {
                order_by_key.reverse()
            }
        })
    });
    visible
}

fn compare_by(left: &FileEntry, right: &FileEntry, key: SortKey) -> Ordering {
    let by_name = || natural_compare(&left.name, &right.name);
    match key {
        SortKey::Name => by_name(),
        SortKey::Size => magnitude(left).cmp(&magnitude(right)).then_with(by_name),
        SortKey::Modified => left.modified.cmp(&right.modified).then_with(by_name),
        SortKey::Kind => natural_compare(&t!(left.kind.label_key()), &t!(right.kind.label_key()))
            .then_with(by_name),
    }
}

fn magnitude(entry: &FileEntry) -> u64 {
    if entry.is_dir {
        entry.child_count.unwrap_or_default() as u64
    } else {
        entry.size.unwrap_or_default()
    }
}
