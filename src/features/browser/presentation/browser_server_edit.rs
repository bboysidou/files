use gpui::{Context, Entity, Focusable, Subscription, Window, prelude::*};
use rust_i18n::t;

use crate::core::dependency_injection::network::ssh_host_source;
use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::core::enums::network::server_field::ServerField;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::browser_event::BrowserEvent;
use crate::features::browser::presentation::state::server_edit::PendingEdit;
use crate::features::browser::presentation::state::server_edit_state::ServerEditState;
use crate::features::network::domain::entities::saved_server::SavedServer;
use crate::features::network::domain::entities::server_form::ServerForm;
use crate::features::network::domain::usecases::server_form_of::server_form_of;
use crate::features::network::domain::usecases::server_form_uri::server_form_uri;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};

impl BrowserView {
    pub fn open_server_edit(
        &mut self,
        original: Option<SavedServer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let form = original.as_ref().map_or_else(
            || ServerForm {
                protocol: NetworkProtocol::Sftp,
                host: String::new(),
                port: String::new(),
                user: String::new(),
                path: String::new(),
            },
            |server| server_form_of(&server.uri),
        );
        let name = original
            .as_ref()
            .and_then(|server| server.name.clone())
            .unwrap_or_default();
        let texts = [form.host, form.port, form.user, form.path, name];
        let (fields, events): (Vec<Entity<TextField>>, Vec<Subscription>) = ServerField::ALL
            .iter()
            .zip(texts)
            .map(|(field, text)| Self::server_field(*field, &text, window, cx))
            .unzip();
        fields[ServerField::Port.index()].update(cx, |input, cx| {
            input.set_placeholder(form.protocol.default_port(), cx)
        });
        window.focus(&fields[ServerField::Host.index()].focus_handle(cx), cx);
        self.server_edit = Some(ServerEditState {
            original,
            protocol: form.protocol,
            fields,
            focus: cx.focus_handle(),
            error: None,
            ssh_hosts: ssh_host_source().hosts(),
            _events: events,
        });
        cx.notify();
    }

    fn server_field(
        field: ServerField,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<TextField>, Subscription) {
        let input = cx.new(|cx| {
            let mut input = TextField::new(t!(field.hint_key()).to_string(), cx);
            input.set_text(text, 0..text.len(), cx);
            input
        });
        let events = cx.subscribe_in(&input, window, |this, _, event, window, cx| match event {
            TextFieldEvent::Changed => {
                if let Some(edit) = this.server_edit.as_mut() {
                    edit.error = None;
                }
                cx.notify();
            }
            TextFieldEvent::Submitted => this.submit_server_edit(window, cx),
            TextFieldEvent::Cancelled => this.close_server_edit(window, cx),
        });
        (input, events)
    }

    pub(super) fn close_server_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.server_edit = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn step_server_field(
        &mut self,
        delta: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(edit) = self.server_edit.as_ref() else {
            return;
        };
        let count = edit.fields.len();
        let current = edit
            .fields
            .iter()
            .position(|field| field.focus_handle(cx).is_focused(window))
            .unwrap_or_default();
        let next = &edit.fields[(current + delta) % count];
        window.focus(&next.focus_handle(cx), cx);
    }

    pub(super) fn step_server_protocol(&mut self, delta: usize, cx: &mut Context<Self>) {
        let Some(edit) = self.server_edit.as_ref() else {
            return;
        };
        let all = NetworkProtocol::ALL;
        let current = all
            .iter()
            .position(|protocol| *protocol == edit.protocol)
            .unwrap_or_default();
        self.select_server_protocol((current + delta) % all.len(), cx);
    }

    pub(super) fn select_server_protocol(&mut self, index: usize, cx: &mut Context<Self>) {
        if let (Some(edit), Some(protocol)) =
            (self.server_edit.as_mut(), NetworkProtocol::ALL.get(index))
        {
            edit.protocol = *protocol;
            edit.error = None;
            edit.fields[ServerField::Port.index()].update(cx, |input, cx| {
                input.set_placeholder(protocol.default_port(), cx)
            });
            cx.notify();
        }
    }

    pub(super) fn server_form(edit: &ServerEditState, cx: &Context<Self>) -> ServerForm {
        let text = |field: ServerField| edit.fields[field.index()].read(cx).text().to_owned();
        ServerForm {
            protocol: edit.protocol,
            host: text(ServerField::Host),
            port: text(ServerField::Port),
            user: text(ServerField::User),
            path: text(ServerField::Path),
        }
    }

    fn submit_server_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(edit) = self.server_edit.as_mut() else {
            return;
        };
        let uri = match server_form_uri(&Self::server_form(edit, cx)) {
            Ok(uri) => uri,
            Err(error) => {
                edit.error = Some(error);
                return cx.notify();
            }
        };
        let name = edit.fields[ServerField::Name.index()]
            .read(cx)
            .text()
            .trim()
            .to_owned();
        let name = Some(name).filter(|name| !name.is_empty());
        let original = edit.original.clone();
        self.close_server_edit(window, cx);
        if original.as_ref().is_some_and(|server| server.uri == uri) {
            return cx.emit(BrowserEvent::ServerRenamed(SavedServer { uri, name }));
        }
        let pending = PendingEdit {
            replaces: original.map_or_else(|| uri.clone(), |server| server.uri),
            name,
        };
        self.active_pane().clone().update(cx, |pane, cx| {
            pane.server_edit.pending = Some(pending);
            pane.connect_network(uri, None, cx)
        });
    }
}
