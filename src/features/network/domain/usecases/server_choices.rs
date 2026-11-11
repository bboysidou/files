use crate::core::constants::ssh::{SFTP_URI_PREFIX, SFTP_URI_SUFFIX};
use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::core::enums::network::server_source::ServerSource;
use crate::features::network::domain::entities::saved_server::SavedServer;
use crate::features::network::domain::entities::server_choice::ServerChoice;
use crate::features::network::domain::entities::ssh_host::SshHost;
use crate::features::network::domain::usecases::server_label::server_label;
use crate::features::places::domain::entities::sidebar_pins::SidebarPins;

pub fn server_choices(
    hosts: &[SshHost],
    saved: &[SavedServer],
    pins: &SidebarPins,
    filter: &str,
) -> Vec<ServerChoice> {
    let ssh = hosts.iter().map(|host| ServerChoice {
        label: host.alias.clone(),
        detail: host.user.clone(),
        uri: format!("{SFTP_URI_PREFIX}{}{SFTP_URI_SUFFIX}", host.alias),
        protocol: Some(NetworkProtocol::Sftp),
        source: ServerSource::Ssh,
        pinned: pins.is_ssh_pinned(&host.alias),
    });
    let saved = saved.iter().filter_map(|server| {
        let address = server_label(&server.uri)?;
        let (label, detail) = match &server.name {
            Some(name) => (name.clone(), Some(address)),
            None => (address, None),
        };
        Some(ServerChoice {
            label,
            detail,
            uri: server.uri.clone(),
            protocol: NetworkProtocol::from_uri(&server.uri),
            source: ServerSource::Saved,
            pinned: !pins.is_server_hidden(&server.uri),
        })
    });
    let needle = filter.trim().to_lowercase();
    ssh.chain(saved)
        .filter(|choice| needle.is_empty() || matches(choice, &needle))
        .collect()
}

fn matches(choice: &ServerChoice, needle: &str) -> bool {
    [
        Some(&choice.label),
        choice.detail.as_ref(),
        Some(&choice.uri),
    ]
    .into_iter()
    .flatten()
    .any(|text| text.to_lowercase().contains(needle))
}
