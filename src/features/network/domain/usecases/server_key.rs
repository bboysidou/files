use crate::core::constants::network::SCP_USER_SEPARATOR;

pub fn server_key(label: &str) -> &str {
    label
        .split_once(SCP_USER_SEPARATOR)
        .map_or(label, |(_, rest)| rest)
}
