use std::fs;
use std::path::{Path, PathBuf};

use crate::core::constants::ssh::{
    SSH_COMMENT, SSH_CONFIG_FILE, SSH_DIR, SSH_GIT_USER, SSH_HOME_PREFIX, SSH_KEY_HOST,
    SSH_KEY_INCLUDE, SSH_KEY_MATCH, SSH_KEY_SEPARATOR, SSH_KEY_USER, SSH_MAX_INCLUDE_DEPTH,
    SSH_PATTERN_CHARS, SSH_WILDCARD,
};
use crate::features::network::domain::entities::ssh_host::SshHost;

pub fn ssh_config_hosts(home: &Path) -> Vec<SshHost> {
    let mut hosts: Vec<SshHost> = Vec::new();
    read_config(
        &home.join(SSH_DIR).join(SSH_CONFIG_FILE),
        home,
        &mut hosts,
        0,
    );
    hosts.retain(|host| host.user.as_deref() != Some(SSH_GIT_USER));
    hosts
}

fn read_config(path: &Path, home: &Path, hosts: &mut Vec<SshHost>, depth: usize) {
    let Ok(text) = fs::read_to_string(path) else {
        return;
    };
    let mut block: Vec<usize> = Vec::new();
    for line in text.lines().map(str::trim) {
        let Some((key, value)) = split_line(line) else {
            continue;
        };
        match key.to_ascii_lowercase().as_str() {
            SSH_KEY_HOST => block = open_block(value, hosts),
            SSH_KEY_MATCH => block.clear(),
            SSH_KEY_USER => set_user(&block, value, hosts),
            SSH_KEY_INCLUDE if depth < SSH_MAX_INCLUDE_DEPTH => {
                for included in included_files(value, home) {
                    read_config(&included, home, hosts, depth + 1);
                }
            }
            _ => {}
        }
    }
}

fn split_line(line: &str) -> Option<(&str, &str)> {
    if line.is_empty() || line.starts_with(SSH_COMMENT) {
        return None;
    }
    let split = line.find(|found: char| found.is_whitespace() || found == SSH_KEY_SEPARATOR)?;
    let (key, rest) = line.split_at(split);
    let value =
        rest.trim_start_matches(|found: char| found.is_whitespace() || found == SSH_KEY_SEPARATOR);
    Some((key, value.trim()))
}

fn open_block(value: &str, hosts: &mut Vec<SshHost>) -> Vec<usize> {
    value
        .split_whitespace()
        .filter(|alias| !alias.contains(SSH_PATTERN_CHARS))
        .map(|alias| {
            hosts
                .iter()
                .position(|host| host.alias == alias)
                .unwrap_or_else(|| {
                    hosts.push(SshHost {
                        alias: alias.to_owned(),
                        user: None,
                    });
                    hosts.len() - 1
                })
        })
        .collect()
}

fn set_user(block: &[usize], value: &str, hosts: &mut [SshHost]) {
    for index in block {
        if let Some(host) = hosts.get_mut(*index) {
            host.user.get_or_insert_with(|| value.to_owned());
        }
    }
}

fn included_files(value: &str, home: &Path) -> Vec<PathBuf> {
    value
        .split_whitespace()
        .flat_map(|pattern| expand(&resolve(pattern, home)))
        .collect()
}

fn resolve(pattern: &str, home: &Path) -> PathBuf {
    if let Some(rest) = pattern.strip_prefix(SSH_HOME_PREFIX) {
        return home.join(rest);
    }
    let path = PathBuf::from(pattern);
    if path.is_absolute() {
        return path;
    }
    home.join(SSH_DIR).join(path)
}

fn expand(path: &Path) -> Vec<PathBuf> {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned());
    let (Some(name), Some(parent)) = (name, path.parent()) else {
        return Vec::new();
    };
    let Some((prefix, suffix)) = name.split_once(SSH_WILDCARD) else {
        return vec![path.to_path_buf()];
    };
    let Ok(entries) = fs::read_dir(parent) else {
        return Vec::new();
    };
    let mut found: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|candidate| {
            candidate.file_name().is_some_and(|file| {
                let file = file.to_string_lossy();
                file.starts_with(prefix) && file.ends_with(suffix)
            })
        })
        .collect();
    found.sort();
    found
}
