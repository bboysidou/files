use gpui::SharedString;

use crate::core::enums::network::network_protocol::NetworkProtocol;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeviceItem {
    pub label: SharedString,
    pub mounted: bool,
    pub active: bool,
    pub saved: bool,
    pub protocol: Option<NetworkProtocol>,
}
