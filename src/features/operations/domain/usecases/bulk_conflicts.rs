use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::core::constants::files::{FORBIDDEN_NAME_CHARS, RESERVED_NAMES};

pub fn bulk_conflicts(
    sources: &[PathBuf],
    names: &[String],
    taken: &dyn Fn(&Path) -> bool,
) -> Vec<bool> {
    let targets: Vec<Option<PathBuf>> = sources
        .iter()
        .zip(names)
        .map(|(source, name)| source.parent().map(|parent| parent.join(name)))
        .collect();
    let mut counts: HashMap<&PathBuf, usize> = HashMap::new();
    for target in targets.iter().flatten() {
        *counts.entry(target).or_default() += 1;
    }
    sources
        .iter()
        .zip(names)
        .zip(&targets)
        .map(|((source, name), target)| {
            let Some(target) = target else {
                return true;
            };
            let invalid = name.trim().is_empty()
                || name.contains(FORBIDDEN_NAME_CHARS)
                || RESERVED_NAMES.contains(&name.as_str());
            let repeated = counts.get(target).copied().unwrap_or_default() > 1;
            invalid || repeated || (target != source && taken(target))
        })
        .collect()
}
