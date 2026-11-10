use crate::core::constants::network::{
    SCHEME_SEPARATOR, SCP_HOST_SEPARATOR, SCP_USER_SEPARATOR, URI_PATH_SEPARATOR,
};
use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::features::network::domain::entities::server_form::ServerForm;

pub fn server_form_of(uri: &str) -> ServerForm {
    let protocol = NetworkProtocol::from_uri(uri).unwrap_or(NetworkProtocol::Sftp);
    let rest = uri
        .split_once(SCHEME_SEPARATOR)
        .map_or(uri, |(_, rest)| rest);
    let (authority, path) = rest.split_once(URI_PATH_SEPARATOR).unwrap_or((rest, ""));
    let (user, address) = authority
        .rsplit_once(SCP_USER_SEPARATOR)
        .map_or(("", authority), |(user, address)| (user, address));
    let (host, port) = address
        .rsplit_once(SCP_HOST_SEPARATOR)
        .filter(|(_, port)| port.chars().all(|found| found.is_ascii_digit()))
        .unwrap_or((address, ""));
    ServerForm {
        protocol,
        host: host.to_owned(),
        port: port.to_owned(),
        user: user.to_owned(),
        path: path.trim_end_matches(URI_PATH_SEPARATOR).to_owned(),
    }
}
