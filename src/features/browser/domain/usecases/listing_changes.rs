use std::collections::{HashMap, HashSet};

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::listing_changes::ListingChanges;

pub fn listing_changes(before: &[FileEntry], after: &[FileEntry]) -> ListingChanges {
    let earlier: HashMap<_, _> = before
        .iter()
        .map(|entry| (&entry.path, (entry.size, entry.modified)))
        .collect();
    let mut changes = ListingChanges::default();
    for entry in after {
        match earlier.get(&entry.path) {
            None => changes.added += 1,
            Some(state) if *state != (entry.size, entry.modified) => changes.changed += 1,
            Some(_) => {}
        }
    }
    let later: HashSet<_> = after.iter().map(|entry| &entry.path).collect();
    changes.removed = before
        .iter()
        .filter(|entry| !later.contains(&entry.path))
        .count();
    changes
}
