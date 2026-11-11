use std::rc::Rc;

use gpui::{AnyElement, Context, Focusable, SharedString, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::core::enums::network::server_field::ServerField;
use crate::core::theme::colors::FG_MUTED;
use crate::core::theme::metrics::TOOLBAR_GAP;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::ssh_host_chip::SshHostChip;
use crate::features::browser::presentation::state::pins::Pins;
use crate::features::browser::presentation::state::server_edit_state::ServerEditState;
use crate::global_components::handlers::IndexHandler;

const DOUBLE: f32 = 2.0;

impl BrowserView {
    fn picked_ssh_host(edit: &ServerEditState, cx: &Context<Self>) -> Option<usize> {
        if edit.protocol != NetworkProtocol::Sftp {
            return None;
        }
        let host = edit.fields[ServerField::Host.index()].read(cx).text();
        edit.ssh_hosts.iter().position(|ssh| ssh.alias == host)
    }

    pub(super) fn pick_ssh_host(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(edit) = self.server_edit.as_ref() else {
            return;
        };
        let Some(alias) = edit.ssh_hosts.get(index).map(|host| host.alias.clone()) else {
            return;
        };
        let host = edit.fields[ServerField::Host.index()].clone();
        let blanks =
            [ServerField::Port, ServerField::User].map(|field| edit.fields[field.index()].clone());
        let sftp = NetworkProtocol::ALL
            .iter()
            .position(|protocol| *protocol == NetworkProtocol::Sftp)
            .unwrap_or_default();
        self.select_server_protocol(sftp, cx);
        host.update(cx, |input, cx| {
            input.set_text(&alias, alias.len()..alias.len(), cx)
        });
        for field in blanks {
            field.update(cx, |input, cx| input.set_text("", 0..0, cx));
        }
        window.focus(&host.focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn step_ssh_host(
        &mut self,
        forward: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(edit) = self.server_edit.as_ref() else {
            return;
        };
        let count = edit.ssh_hosts.len();
        if count == 0 || edit.original.is_some() {
            return;
        }
        let next = match (Self::picked_ssh_host(edit, cx), forward) {
            (None, true) => 0,
            (None, false) => count - 1,
            (Some(index), true) => (index + 1) % count,
            (Some(index), false) => (index + count - 1) % count,
        };
        self.pick_ssh_host(next, window, cx);
    }

    pub(super) fn toggle_ssh_pin(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(alias) = self
            .server_edit
            .as_ref()
            .and_then(|edit| edit.ssh_hosts.get(index))
            .map(|host| host.alias.clone())
        else {
            return;
        };
        cx.global_mut::<Pins>().0.toggle_ssh(&alias);
        cx.refresh_windows();
        cx.notify();
    }

    pub(super) fn toggle_picked_ssh_pin(&mut self, cx: &mut Context<Self>) {
        if let Some(index) = self
            .server_edit
            .as_ref()
            .and_then(|edit| Self::picked_ssh_host(edit, cx))
        {
            self.toggle_ssh_pin(index, cx);
        }
    }

    fn ssh_chip_handlers(cx: &mut Context<Self>) -> (IndexHandler, IndexHandler) {
        let (pick_view, pin_view) = (cx.entity().downgrade(), cx.entity().downgrade());
        let on_pick: IndexHandler = Rc::new(move |index, window, cx| {
            let _ = pick_view.update(cx, |this, cx| this.pick_ssh_host(index, window, cx));
        });
        let on_pin: IndexHandler = Rc::new(move |index, _, cx| {
            cx.stop_propagation();
            let _ = pin_view.update(cx, |this, cx| this.toggle_ssh_pin(index, cx));
        });
        (on_pick, on_pin)
    }

    pub(super) fn server_ssh_hosts(
        edit: &ServerEditState,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if edit.ssh_hosts.is_empty() || edit.original.is_some() {
            return None;
        }
        let picked = Self::picked_ssh_host(edit, cx);
        let (on_pick, on_pin) = Self::ssh_chip_handlers(cx);
        let pins = cx
            .try_global::<Pins>()
            .map(|pins| pins.0.clone())
            .unwrap_or_default();
        let chips = edit
            .ssh_hosts
            .iter()
            .enumerate()
            .map(|(index, host)| SshHostChip {
                index,
                alias: SharedString::from(host.alias.clone()),
                user: host.user.clone().map(SharedString::from),
                selected: picked == Some(index),
                pinned: pins.is_ssh_pinned(&host.alias),
                on_pick: on_pick.clone(),
                on_pin: on_pin.clone(),
            });
        Some(
            div()
                .flex()
                .flex_col()
                .gap(px(TOOLBAR_GAP / DOUBLE))
                .child(
                    div()
                        .text_color(rgb(FG_MUTED))
                        .child(t!("server_edit.ssh_hosts").to_string()),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(TOOLBAR_GAP / DOUBLE))
                        .children(chips),
                )
                .into_any_element(),
        )
    }
}
