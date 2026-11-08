use crate::core::constants::network::{
    GVFS_HOST_KEY, GVFS_PAIR_SEPARATOR, GVFS_PORT_KEY, GVFS_SCHEME_SEPARATOR, GVFS_SHARE_KEY,
    GVFS_USER_KEY, GVFS_VALUE_SEPARATOR,
};

pub fn network_mount_label(name: &str) -> Option<String> {
    let (_, pairs) = name.split_once(GVFS_SCHEME_SEPARATOR)?;
    let value = |key: &str| {
        pairs
            .split(GVFS_PAIR_SEPARATOR)
            .filter_map(|pair| pair.split_once(GVFS_VALUE_SEPARATOR))
            .find(|(found, _)| *found == key)
            .map(|(_, value)| value)
    };
    let host = value(GVFS_HOST_KEY)?;
    let user = value(GVFS_USER_KEY).map_or(String::new(), |user| format!("{user}@"));
    let port = value(GVFS_PORT_KEY).map_or(String::new(), |port| format!(":{port}"));
    let share = value(GVFS_SHARE_KEY).map_or(String::new(), |share| format!("/{share}"));
    Some(format!("{user}{host}{port}{share}"))
}
