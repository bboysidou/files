use gpui::Context;

use crate::features::browser::presentation::state::browser_event::BrowserEvent;
use crate::features::network::domain::usecases::server_label::server_label;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::network_entry::NetworkEntry;

impl ShellView {
    pub(super) fn watch_servers(&mut self, cx: &mut Context<Self>) {
        self.network.saved = self.saved.servers.clone();
        let browser = self.browser.clone();
        self.network.events = Some(cx.subscribe(&browser, |this, _, event, cx| match event {
            BrowserEvent::ServerConnected(uri) => this.remember_server(uri, cx),
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
                    .find(|mount| mount.label == label)
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
            .filter(|mount| !entries.iter().any(|entry| entry.label == mount.label))
            .map(|mount| NetworkEntry {
                label: mount.label.clone(),
                uri: None,
                mount: Some(mount.path.clone()),
            })
            .collect();
        entries.extend(unsaved);
        entries
    }

    fn remember_server(&mut self, uri: &str, cx: &mut Context<Self>) {
        let label = server_label(uri);
        if label.is_none() {
            return;
        }
        self.network
            .saved
            .retain(|saved| server_label(saved) != label);
        self.network.saved.push(uri.to_owned());
        self.refresh_network(cx);
        self.persist(cx);
        cx.notify();
    }

    pub(super) fn forget_server(&mut self, uri: &str, cx: &mut Context<Self>) {
        self.network.saved.retain(|saved| saved != uri);
        self.persist(cx);
        cx.notify();
    }
}
