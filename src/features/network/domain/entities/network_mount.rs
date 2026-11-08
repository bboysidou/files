use std::path::PathBuf;

use crate::core::enums::network::network_protocol::NetworkProtocol;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkMount {
    pub label: String,
    pub path: PathBuf,
    pub protocol: Option<NetworkProtocol>,
}
