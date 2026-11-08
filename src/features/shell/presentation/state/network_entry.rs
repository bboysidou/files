use std::path::PathBuf;

use crate::core::enums::network::network_protocol::NetworkProtocol;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkEntry {
    pub label: String,
    pub uri: Option<String>,
    pub mount: Option<PathBuf>,
    pub protocol: Option<NetworkProtocol>,
}
