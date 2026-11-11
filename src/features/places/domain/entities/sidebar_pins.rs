use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SidebarPins {
    pub favorites: Vec<PathBuf>,
    pub ssh: Vec<String>,
    pub hidden_servers: Vec<String>,
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

    pub fn is_server_hidden(&self, uri: &str) -> bool {
        self.hidden_servers.iter().any(|hidden| hidden == uri)
    }

    pub fn toggle_server(&mut self, uri: &str) {
        match self.hidden_servers.iter().position(|hidden| hidden == uri) {
            Some(index) => {
                self.hidden_servers.remove(index);
            }
            None => self.hidden_servers.push(uri.to_owned()),
        }
    }
}
