use std::path::{Component, Path};

use super::place::PlaceTarget;
use crate::core::constants::paths::PATH_SEPARATOR;
use crate::core::enums::places::crumb_root::CrumbRoot;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CrumbTrail {
    pub root: CrumbRoot,
    pub segments: Vec<String>,
}

impl CrumbTrail {
    pub fn for_target(target: &PlaceTarget, home: Option<&Path>) -> Self {
        match target {
            PlaceTarget::Overview => Self::bare(CrumbRoot::Overview),
            PlaceTarget::Trash => Self::bare(CrumbRoot::Trash),
            PlaceTarget::Directory(path) => Self::for_directory(path, home),
        }
    }

    pub fn display_path(&self) -> String {
        let joined = self.segments.join(PATH_SEPARATOR);
        match (self.root, self.segments.is_empty()) {
            (_, true) => self.root.text().into_owned(),
            (CrumbRoot::Filesystem, false) => format!("{PATH_SEPARATOR}{joined}"),
            (root, false) => format!("{}{PATH_SEPARATOR}{joined}", root.text()),
        }
    }

    fn bare(root: CrumbRoot) -> Self {
        Self {
            root,
            segments: Vec::new(),
        }
    }

    fn for_directory(path: &Path, home: Option<&Path>) -> Self {
        match home.and_then(|home| path.strip_prefix(home).ok()) {
            Some(rest) => Self {
                root: CrumbRoot::Home,
                segments: segments_of(rest),
            },
            None => Self {
                root: CrumbRoot::Filesystem,
                segments: segments_of(path),
            },
        }
    }
}

fn segments_of(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(name) => Some(name.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect()
}
