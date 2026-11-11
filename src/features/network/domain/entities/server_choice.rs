use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::core::enums::network::server_source::ServerSource;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerChoice {
    pub label: String,
    pub detail: Option<String>,
    pub uri: String,
    pub protocol: Option<NetworkProtocol>,
    pub source: ServerSource,
    pub pinned: bool,
}
