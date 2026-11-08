use gpui::{Context, Window};

use crate::features::browser::presentation::state::browser_event::BrowserEvent;
use crate::features::network::domain::usecases::server_key::server_key;
use crate::features::network::domain::usecases::server_label::server_label;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::network_entry::NetworkEntry;

impl ShellView {
    pub(super) fn watch_servers(&mut self, cx: &mut Context<Self>) {
        self.network.saved = self.saved.servers.clone();
        let browser = self.browser.clone();
        self.network.events = Some(cx.subscribe(&browser, |this, _, event, cx| match event {
            BrowserEvent::ServerConnected { uri, replaces } => {
                this.remember_server(uri, replaces.as_deref(), cx)
            }
        }));
    }

    pub(super) fn network_entries(&self) -> Vec<NetworkEntry> {
        let mut entries: Vec<NetworkEntry> = self
            .network
            .saved
            .iter()
            .filter_map(|uri| {
                let label = server_label(uri)?;
                let mount = self
                    .network
                    .mounts
                    .iter()
                    .find(|mount| server_key(&mount.label) == server_key(&label))
                    .map(|mount| mount.path.clone());
                Some(NetworkEntry {
                    label,
                    uri: Some(uri.clone()),
                    mount,
                })
            })
            .collect();
        let unsaved: Vec<NetworkEntry> = self
            .network
            .mounts
            .iter()
            .filter(|mount| {
                !entries
                    .iter()
                    .any(|entry| server_key(&entry.label) == server_key(&mount.label))
            })
            .map(|mount| NetworkEntry {
                label: mount.label.clone(),
                uri: None,
                mount: Some(mount.path.clone()),
            })
            .collect();
        entries.extend(unsaved);
        entries
    }

    fn remember_server(&mut self, uri: &str, replaces: Option<&str>, cx: &mut Context<Self>) {
        let label = server_label(uri);
        if label.is_none() {
            return;
        }
        let slot = replaces
            .and_then(|old| self.network.saved.iter().position(|saved| saved == old))
            .unwrap_or(self.network.saved.len());
        let mut saved: Vec<String> = self.network.saved.clone();
        saved.insert(slot, uri.to_owned());
        let mut seen = Vec::new();
        saved.retain(|entry| {
            let keep = Some(entry.as_str()) != replaces || entry == uri;
            let fresh = !seen.contains(&server_label(entry));
            seen.push(server_label(entry));
            keep && fresh
        });
        self.network.saved = saved;
        self.refresh_network(cx);
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
            .network_entries()
            .into_iter()
            .nth(index)
            .and_then(|entry| entry.uri)
        else {
            return;
        };
        self.browser
            .update(cx, |browser, cx| browser.edit_server(uri, window, cx));
    }

    pub(super) fn forget_server(&mut self, uri: &str, cx: &mut Context<Self>) {
        self.network.saved.retain(|saved| saved != uri);
        self.persist(cx);
        cx.notify();
    }
}
