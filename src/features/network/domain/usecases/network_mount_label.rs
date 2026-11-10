use crate::core::constants::network::{
    GVFS_HOST_KEY, GVFS_PORT_KEY, GVFS_SCHEME_SEPARATOR, GVFS_SERVER_KEY, GVFS_SHARE_KEY,
    GVFS_USER_KEY,
};
use crate::core::constants::phones::MTP_SCHEME;
use crate::features::network::domain::usecases::gvfs_mount_value::gvfs_mount_value;
use crate::features::phones::domain::usecases::mtp_label::mtp_label;

pub fn network_mount_label(name: &str) -> Option<String> {
    let (scheme, _) = name.split_once(GVFS_SCHEME_SEPARATOR)?;
    let value = |key: &str| gvfs_mount_value(name, key);
    let host = value(GVFS_HOST_KEY).or_else(|| value(GVFS_SERVER_KEY))?;
    if scheme == MTP_SCHEME {
        return Some(mtp_label(host));
    }
    let user = value(GVFS_USER_KEY).map_or(String::new(), |user| format!("{user}@"));
    let port = value(GVFS_PORT_KEY).map_or(String::new(), |port| format!(":{port}"));
    let share = value(GVFS_SHARE_KEY).map_or(String::new(), |share| format!("/{share}"));
    Some(format!("{user}{host}{port}{share}"))
}
