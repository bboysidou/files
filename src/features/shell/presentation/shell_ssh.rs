use std::rc::Rc;

use gpui::{Context, SharedString};

use crate::core::constants::ssh::{SFTP_URI_PREFIX, SFTP_URI_SUFFIX};
use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::features::network::domain::entities::network_mount::NetworkMount;
use crate::features::network::domain::usecases::server_key::server_key;
use crate::features::network::domain::usecases::server_label::server_label;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::device_item::DeviceItem;
use crate::features::shell::presentation::state::network_entry::NetworkEntry;
use crate::global_components::handlers::IndexHandler;

impl ShellView {
    pub(super) fn ssh_entries(&self) -> Vec<NetworkEntry> {
        self.network
            .ssh_hosts
            .iter()
            .map(|host| NetworkEntry {
                label: host.alias.clone(),
                uri: Some(format!("{SFTP_URI_PREFIX}{}{SFTP_URI_SUFFIX}", host.alias)),
                mount: self
                    .network
                    .mounts
                    .iter()
                    .find(|mount| self.ssh_alias_of(mount) == Some(host.alias.as_str()))
                    .map(|mount| mount.path.clone()),
                protocol: Some(NetworkProtocol::Sftp),
            })
            .collect()
    }

    pub(super) fn ssh_items(&self, location: &PlaceTarget) -> Vec<DeviceItem> {
        self.ssh_entries()
            .into_iter()
            .map(|entry| DeviceItem {
                active: matches!((location, &entry.mount), (PlaceTarget::Directory(path), Some(mount)) if path.starts_with(mount)),
                label: SharedString::from(entry.label),
                mounted: entry.mount.is_some(),
                saved: false,
                protocol: None,
            })
            .collect()
    }

    pub(super) fn is_ssh_mount(&self, mount: &NetworkMount) -> bool {
        self.ssh_alias_of(mount).is_some()
    }

    pub(super) fn is_ssh_uri(&self, uri: &str) -> bool {
        NetworkProtocol::from_uri(uri) == Some(NetworkProtocol::Sftp)
            && server_label(uri).is_some_and(|label| self.is_ssh_alias(server_key(&label)))
    }

    fn ssh_alias_of<'a>(&self, mount: &'a NetworkMount) -> Option<&'a str> {
        let key = server_key(&mount.label);
        (mount.protocol == Some(NetworkProtocol::Sftp) && self.is_ssh_alias(key)).then_some(key)
    }

    fn is_ssh_alias(&self, key: &str) -> bool {
        self.network.ssh_hosts.iter().any(|host| host.alias == key)
    }

    pub(super) fn open_ssh(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.ssh_entries().into_iter().nth(index) else {
            return;
        };
        match (entry.mount, entry.uri) {
            (Some(mount), _) => self.browser.update(cx, |browser, cx| {
                browser.navigate(PlaceTarget::Directory(mount), cx)
            }),
            (None, Some(uri)) => self
                .browser
                .update(cx, |browser, cx| browser.connect_server(uri, cx)),
            (None, None) => {}
        }
    }

    pub(super) fn eject_ssh(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.ssh_entries().into_iter().nth(index) else {
            return;
        };
        if let Some(mount) = entry.mount {
            self.unmount_network(entry.label, mount, cx);
        }
    }

    pub(super) fn ssh_handlers(cx: &mut Context<Self>) -> (IndexHandler, IndexHandler) {
        let (open_view, eject_view) = (cx.entity().downgrade(), cx.entity().downgrade());
        let on_open: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = open_view.update(cx, |this, cx| this.open_ssh(index, cx));
        });
        let on_eject: IndexHandler = Rc::new(move |index, _, cx| {
            cx.stop_propagation();
            let _ = eject_view.update(cx, |this, cx| this.eject_ssh(index, cx));
        });
        (on_open, on_eject)
    }
}
