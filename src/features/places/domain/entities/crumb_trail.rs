use std::path::{Component, Path, PathBuf};

use super::place::PlaceTarget;
use crate::core::constants::paths::PATH_SEPARATOR;
use crate::core::enums::places::crumb_root::CrumbRoot;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CrumbTrail {
    pub root: CrumbRoot,
    pub segments: Vec<String>,
    pub targets: Vec<PlaceTarget>,
}

impl CrumbTrail {
    pub fn for_target(target: &PlaceTarget, home: Option<&Path>) -> Self {
        match target {
            PlaceTarget::Overview => Self::bare(CrumbRoot::Overview, target),
            PlaceTarget::Trash => Self::bare(CrumbRoot::Trash, target),
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

    fn bare(root: CrumbRoot, target: &PlaceTarget) -> Self {
        Self {
            root,
            segments: Vec::new(),
            targets: vec![target.clone()],
        }
    }

    fn for_directory(path: &Path, home: Option<&Path>) -> Self {
        let (root, base, rest) =
            match home.and_then(|home| Some((home, path.strip_prefix(home).ok()?))) {
                Some((home, rest)) => (CrumbRoot::Home, home.to_path_buf(), rest),
                None => (CrumbRoot::Filesystem, PathBuf::from(PATH_SEPARATOR), path),
            };
        let segments = segments_of(rest);
        let mut current = base.clone();
        let mut targets = vec![PlaceTarget::Directory(base)];
        for segment in &segments {
            current.push(segment);
            targets.push(PlaceTarget::Directory(current.clone()));
        }
        Self {
            root,
            segments,
            targets,
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
