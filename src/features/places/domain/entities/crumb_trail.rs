use std::path::{Component, Path, PathBuf};

use super::place::PlaceTarget;
use crate::core::constants::paths::PATH_SEPARATOR;
use crate::core::enums::places::crumb_root::CrumbRoot;
use crate::features::network::domain::usecases::network_mount_root::network_mount_root;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CrumbTrail {
    pub root: CrumbRoot,
    pub root_label: Option<String>,
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

    pub fn root_text(&self) -> String {
        self.root_label
            .clone()
            .unwrap_or_else(|| self.root.text().into_owned())
    }

    pub fn display_path(&self) -> String {
        let joined = self.segments.join(PATH_SEPARATOR);
        match (self.root, self.segments.is_empty()) {
            (_, true) => self.root_text(),
            (CrumbRoot::Filesystem, false) => format!("{PATH_SEPARATOR}{joined}"),
            (_, false) => format!("{}{PATH_SEPARATOR}{joined}", self.root_text()),
        }
    }

    fn bare(root: CrumbRoot, target: &PlaceTarget) -> Self {
        Self {
            root,
            root_label: None,
            segments: Vec::new(),
            targets: vec![target.clone()],
        }
    }

    fn for_directory(path: &Path, home: Option<&Path>) -> Self {
        let network = network_mount_root(path);
        let (root, base, rest) = match (&network, home) {
            (Some((mount, _)), _) => (
                CrumbRoot::Network,
                mount.clone(),
                path.strip_prefix(mount).unwrap_or(path),
            ),
            (None, Some(home)) if path.starts_with(home) => (
                CrumbRoot::Home,
                home.to_path_buf(),
                path.strip_prefix(home).unwrap_or(path),
            ),
            _ => (CrumbRoot::Filesystem, PathBuf::from(PATH_SEPARATOR), path),
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
            root_label: network.map(|(_, label)| label),
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
