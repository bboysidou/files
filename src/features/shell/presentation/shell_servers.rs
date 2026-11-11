use gpui::{App, Context, Window};

use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::features::browser::presentation::state::browser_event::BrowserEvent;
use crate::features::browser::presentation::state::pins::Pins;
use crate::features::browser::presentation::state::server_names::ServerNames;
use crate::features::network::domain::entities::saved_server::SavedServer;
use crate::features::network::domain::usecases::server_key::server_key;
use crate::features::network::domain::usecases::server_label::server_label;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::network_entry::NetworkEntry;

impl ShellView {
    pub(super) fn watch_servers(&mut self, cx: &mut Context<Self>) {
        self.network.saved = self.saved.servers.clone();
        self.publish_server_names(cx);
        let browser = self.browser.clone();
        self.network.events = Some(cx.subscribe(&browser, |this, _, event, cx| match event {
            BrowserEvent::ServerConnected { uri, edit } => {
                let server = SavedServer {
                    uri: uri.clone(),
                    name: edit.as_ref().and_then(|edit| edit.name.clone()),
                };
                let replaces = edit.as_ref().map(|edit| edit.replaces.as_str());
                this.remember_server(server, replaces, cx)
            }
            BrowserEvent::ServerRenamed(server) => this.rename_server(server, cx),
            BrowserEvent::ServerForgotten(uri) => this.forget_server(&uri.clone(), cx),
            BrowserEvent::TabDetached(tab) => Self::open_detached(tab.clone(), cx),
        }));
    }

    pub(super) fn network_entries(&self, cx: &App) -> Vec<NetworkEntry> {
        let hidden: Vec<String> = cx
            .try_global::<Pins>()
            .map(|pins| pins.0.hidden_servers.clone())
            .unwrap_or_default();
        let mut entries: Vec<NetworkEntry> = self
            .network
            .saved
            .iter()
            .filter(|server| !hidden.contains(&server.uri))
            .filter_map(|server| {
                let address = server_label(&server.uri)?;
                let mount = self
                    .network
                    .mounts
                    .iter()
                    .find(|mount| server_key(&mount.label) == server_key(&address))
                    .map(|mount| mount.path.clone());
                Some(NetworkEntry {
                    label: server.name.clone().unwrap_or(address),
                    protocol: NetworkProtocol::from_uri(&server.uri),
                    uri: Some(server.uri.clone()),
                    mount,
                })
            })
            .collect();
        let saved_keys: Vec<String> = self
            .network
            .saved
            .iter()
            .filter_map(|server| server_label(&server.uri))
            .collect();
        entries.extend(
            self.network
                .mounts
                .iter()
                .filter(|mount| {
                    !saved_keys
                        .iter()
                        .any(|key| server_key(key) == server_key(&mount.label))
                        && !self.is_ssh_mount(mount)
                })
                .map(|mount| NetworkEntry {
                    label: mount.label.clone(),
                    uri: None,
                    mount: Some(mount.path.clone()),
                    protocol: mount.protocol,
                }),
        );
        entries
    }

    fn remember_server(
        &mut self,
        server: SavedServer,
        replaces: Option<&str>,
        cx: &mut Context<Self>,
    ) {
        let Some(label) = server_label(&server.uri) else {
            return;
        };
        if self.is_ssh_uri(&server.uri) {
            return self.refresh_network(cx);
        }
        let saved = &mut self.network.saved;
        let slot = replaces.and_then(|old| saved.iter().position(|entry| entry.uri == old));
        match slot {
            Some(slot) => saved[slot] = server,
            None if saved
                .iter()
                .any(|entry| server_label(&entry.uri).as_ref() == Some(&label)) => {}
            None => saved.push(server),
        }
        let mut seen = Vec::new();
        saved.retain(|entry| {
            let key = server_label(&entry.uri);
            let fresh = !seen.contains(&key);
            seen.push(key);
            fresh
        });
        self.refresh_network(cx);
        self.publish_server_names(cx);
        self.persist(cx);
        cx.notify();
    }

    fn publish_server_names(&self, cx: &mut Context<Self>) {
        cx.set_global(ServerNames(self.network.saved.clone()));
        cx.refresh_windows();
    }

    fn rename_server(&mut self, server: &SavedServer, cx: &mut Context<Self>) {
        if let Some(entry) = self
            .network
            .saved
            .iter_mut()
            .find(|entry| entry.uri == server.uri)
        {
            entry.name = server.name.clone();
        }
        self.publish_server_names(cx);
        self.persist(cx);
        cx.notify();
    }

    pub(super) fn edit_server(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(uri) = self
            .network_entries(cx)
            .into_iter()
            .nth(index)
            .and_then(|entry| entry.uri)
        else {
            return;
        };
        let Some(server) = self
            .network
            .saved
            .iter()
            .find(|server| server.uri == uri)
            .cloned()
        else {
            return;
        };
        self.browser.update(cx, |browser, cx| {
            browser.open_server_edit(Some(server), window, cx)
        });
    }

    pub(super) fn forget_server(&mut self, uri: &str, cx: &mut Context<Self>) {
        self.network.saved.retain(|saved| saved.uri != uri);
        let pins = &mut cx.global_mut::<Pins>().0;
        if pins.is_server_hidden(uri) {
            pins.toggle_server(uri);
        }
        self.publish_server_names(cx);
        self.persist(cx);
        cx.notify();
    }
}
