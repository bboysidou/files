use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SidebarPins {
    pub favorites: Vec<PathBuf>,
    pub ssh: Vec<String>,
}

impl SidebarPins {
    pub fn is_favorite(&self, path: &Path) -> bool {
        self.favorites.iter().any(|favorite| favorite == path)
    }

    pub fn is_ssh_pinned(&self, alias: &str) -> bool {
        self.ssh.iter().any(|pinned| pinned == alias)
    }

    pub fn toggle_favorite(&mut self, path: PathBuf) {
        match self.favorites.iter().position(|favorite| *favorite == path) {
            Some(index) => {
                self.favorites.remove(index);
            }
            None => self.favorites.push(path),
        }
    }

    pub fn toggle_ssh(&mut self, alias: &str) {
        match self.ssh.iter().position(|pinned| pinned == alias) {
            Some(index) => {
                self.ssh.remove(index);
            }
            None => self.ssh.push(alias.to_owned()),
        }
    }
}
