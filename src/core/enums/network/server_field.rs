use crate::core::enums::network::network_protocol::NetworkProtocol;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ServerField {
    Host,
    Port,
    User,
    Path,
    Name,
}

impl ServerField {
    pub const ALL: [ServerField; 5] = [
        ServerField::Host,
        ServerField::Port,
        ServerField::User,
        ServerField::Path,
        ServerField::Name,
    ];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn label_key(self, protocol: NetworkProtocol) -> &'static str {
        match self {
            ServerField::Host => "server_edit.host",
            ServerField::Port => "server_edit.port",
            ServerField::User => "server_edit.user",
            ServerField::Path => protocol.path_label_key(),
            ServerField::Name => "server_edit.name",
        }
    }

    pub fn hint_key(self) -> &'static str {
        match self {
            ServerField::Host => "server_edit.host_hint",
            ServerField::Port | ServerField::User | ServerField::Path => "server_edit.optional",
            ServerField::Name => "server_edit.name_hint",
        }
    }
}
