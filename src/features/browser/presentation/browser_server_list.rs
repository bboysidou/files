use gpui::AppContext;
use gpui::{
    Context, Entity, Focusable, ScrollStrategy, Subscription, UniformListScrollHandle, Window,
};
use rust_i18n::t;

use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::core::enums::network::server_field::ServerField;
use crate::core::enums::network::server_source::ServerSource;
use crate::core::enums::network::server_tab::ServerTab;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::browser_event::BrowserEvent;
use crate::features::browser::presentation::state::pins::Pins;
use crate::features::browser::presentation::state::server_edit_state::ServerEditState;
use crate::features::browser::presentation::state::server_list_state::ServerListState;
use crate::features::browser::presentation::state::server_names::ServerNames;
use crate::features::network::domain::entities::server_choice::ServerChoice;
use crate::features::network::domain::entities::ssh_host::SshHost;
use crate::features::network::domain::usecases::server_choices::server_choices;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};

impl BrowserView {
    pub(super) fn server_list(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (ServerListState, Subscription) {
        let filter = cx.new(|cx| TextField::new(t!("server_edit.filter").to_string(), cx));
        let events = cx.subscribe_in(&filter, window, |this, _, event, window, cx| match event {
            TextFieldEvent::Changed => {
                if let Some(edit) = this.server_edit.as_mut() {
                    edit.list.cursor = 0;
                    edit.list.scroll.scroll_to_item(0, ScrollStrategy::Top);
                }
                cx.notify();
            }
            TextFieldEvent::Submitted => this.connect_server_choice(None, window, cx),
            TextFieldEvent::Cancelled => this.close_server_edit(window, cx),
        });
        let list = ServerListState {
            filter,
            cursor: 0,
            scroll: UniformListScrollHandle::new(),
        };
        (list, events)
    }

    pub(super) fn has_server_choices(hosts: &[SshHost], cx: &Context<Self>) -> bool {
        !hosts.is_empty()
            || cx
                .try_global::<ServerNames>()
                .is_some_and(|names| !names.0.is_empty())
    }

    pub(super) fn server_choice_list(
        edit: &ServerEditState,
        cx: &Context<Self>,
    ) -> Vec<ServerChoice> {
        let saved = cx
            .try_global::<ServerNames>()
            .map(|names| names.0.clone())
            .unwrap_or_default();
        let pins = cx
            .try_global::<Pins>()
            .map(|pins| pins.0.clone())
            .unwrap_or_default();
        let filter = edit.list.filter.read(cx).text();
        server_choices(&edit.ssh_hosts, &saved, &pins, filter)
    }

    fn chosen(&self, index: Option<usize>, cx: &Context<Self>) -> Option<ServerChoice> {
        let edit = self.server_edit.as_ref()?;
        let index = index.unwrap_or(edit.list.cursor);
        Self::server_choice_list(edit, cx).into_iter().nth(index)
    }

    pub(super) fn focus_server_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(edit) = self.server_edit.as_ref() else {
            return;
        };
        let field: Entity<TextField> = match edit.tab {
            ServerTab::Servers => edit.list.filter.clone(),
            ServerTab::New => edit.fields[ServerField::Host.index()].clone(),
        };
        window.focus(&field.focus_handle(cx), cx);
    }

    pub(super) fn switch_server_tab(
        &mut self,
        tab: ServerTab,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(edit) = self
            .server_edit
            .as_mut()
            .filter(|edit| edit.original.is_none())
        else {
            return;
        };
        edit.tab = tab;
        self.focus_server_tab(window, cx);
        cx.notify();
    }

    pub(super) fn toggle_server_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(tab) = self.server_edit.as_ref().map(|edit| edit.tab.other()) {
            self.switch_server_tab(tab, window, cx);
        }
    }

    pub(super) fn step_server_choice(&mut self, delta: isize, cx: &mut Context<Self>) {
        let Some(edit) = self.server_edit.as_ref() else {
            return;
        };
        let Some(last) = Self::server_choice_list(edit, cx).len().checked_sub(1) else {
            return;
        };
        let Some(edit) = self.server_edit.as_mut() else {
            return;
        };
        edit.list.cursor = (edit.list.cursor as isize + delta).clamp(0, last as isize) as usize;
        edit.list
            .scroll
            .scroll_to_item(edit.list.cursor, ScrollStrategy::Nearest);
        cx.notify();
    }

    pub(super) fn connect_server_choice(
        &mut self,
        index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(choice) = self.chosen(index, cx) else {
            return;
        };
        self.close_server_edit(window, cx);
        self.connect_server(choice.uri, cx);
    }

    pub(super) fn toggle_server_pin(&mut self, index: Option<usize>, cx: &mut Context<Self>) {
        let Some(choice) = self.chosen(index, cx) else {
            return;
        };
        let pins = &mut cx.global_mut::<Pins>().0;
        match choice.source {
            ServerSource::Ssh => pins.toggle_ssh(&choice.label),
            ServerSource::Saved => pins.toggle_server(&choice.uri),
        }
        cx.refresh_windows();
        cx.notify();
    }

    pub(super) fn forget_server_choice(&mut self, index: Option<usize>, cx: &mut Context<Self>) {
        if let Some(choice) = self
            .chosen(index, cx)
            .filter(|choice| choice.source == ServerSource::Saved)
        {
            cx.emit(BrowserEvent::ServerForgotten(choice.uri));
        }
    }

    pub(super) fn edit_server_choice(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(choice) = self.chosen(None, cx) else {
            return;
        };
        if choice.source == ServerSource::Saved {
            let server = cx.try_global::<ServerNames>().and_then(|names| {
                names
                    .0
                    .iter()
                    .find(|server| server.uri == choice.uri)
                    .cloned()
            });
            if let Some(server) = server {
                self.open_server_edit(Some(server), window, cx);
            }
            return;
        }
        self.prefill_ssh(&choice.label, window, cx);
    }

    fn prefill_ssh(&mut self, alias: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(edit) = self.server_edit.as_mut() else {
            return;
        };
        edit.tab = ServerTab::New;
        let host = edit.fields[ServerField::Host.index()].clone();
        let blanks =
            [ServerField::Port, ServerField::User].map(|field| edit.fields[field.index()].clone());
        let ssh = NetworkProtocol::ALL
            .iter()
            .position(|protocol| *protocol == NetworkProtocol::Ssh)
            .unwrap_or_default();
        self.select_server_protocol(ssh, cx);
        host.update(cx, |input, cx| {
            input.set_text(alias, alias.len()..alias.len(), cx)
        });
        for field in blanks {
            field.update(cx, |input, cx| input.set_text("", 0..0, cx));
        }
        self.focus_server_tab(window, cx);
        cx.notify();
    }
}
