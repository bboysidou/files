use crate::core::constants::network::{
    GVFS_PAIR_SEPARATOR, GVFS_SCHEME_SEPARATOR, GVFS_VALUE_SEPARATOR,
};

pub fn gvfs_mount_value<'a>(name: &'a str, key: &str) -> Option<&'a str> {
    let (_, pairs) = name.split_once(GVFS_SCHEME_SEPARATOR)?;
    pairs
        .split(GVFS_PAIR_SEPARATOR)
        .filter_map(|pair| pair.split_once(GVFS_VALUE_SEPARATOR))
        .find(|(found, _)| *found == key)
        .map(|(_, value)| value)
}
