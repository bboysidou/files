use crate::core::enums::network::network_protocol::NetworkProtocol;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerForm {
    pub protocol: NetworkProtocol,
    pub host: String,
    pub port: String,
    pub user: String,
    pub path: String,
}
