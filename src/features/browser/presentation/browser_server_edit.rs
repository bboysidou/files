use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Entity, Focusable, FontWeight, IntoElement,
    MouseButton, Subscription, Window, anchored, deferred, div, point, prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::theme::colors::{CARD, FG_BRIGHT, FG_MUTED, LINE_STRONG};
use crate::core::theme::metrics::{
    QUICK_LOOK_MARGIN, QUICK_LOOK_RADIUS, SETTINGS_PANEL_WIDTH, SHEET_PANEL_PAD, TEXT_EMPTY_TITLE,
    TOOLBAR_GAP,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::key_bindings::SERVER_EDIT_CONTEXT;
use crate::features::browser::presentation::state::browser_event::BrowserEvent;
use crate::features::browser::presentation::state::server_edit::PendingEdit;
use crate::features::browser::presentation::state::server_edit_state::ServerEditState;
use crate::features::network::domain::entities::saved_server::SavedServer;
use crate::features::network::domain::usecases::network_uri::network_uri;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::modal_scrim::ModalScrim;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};

const EDIT_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;
const NAME_FIELD: usize = 0;
const ADDRESS_FIELD: usize = 1;
const FIELD_LABELS: [&str; 2] = ["server_edit.name", "server_edit.address"];

impl BrowserView {
    pub fn open_server_edit(
        &mut self,
        server: SavedServer,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name = server.name.clone().unwrap_or_default();
        let (name_field, name_events) = Self::server_field(&name, window, cx);
        let (address_field, address_events) = Self::server_field(&server.uri, window, cx);
        window.focus(&name_field.focus_handle(cx), cx);
        self.server_edit = Some(ServerEditState {
            original: server,
            fields: [name_field, address_field],
            focus: cx.focus_handle(),
            _events: [name_events, address_events],
        });
        cx.notify();
    }

    fn server_field(
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<TextField>, Subscription) {
        let field = cx.new(|cx| {
            let mut field = TextField::new(String::new(), cx);
            field.set_text(text, 0..text.len(), cx);
            field
        });
        let events = cx.subscribe_in(&field, window, |this, _, event, window, cx| match event {
            TextFieldEvent::Changed => {}
            TextFieldEvent::Submitted => this.submit_server_edit(window, cx),
            TextFieldEvent::Cancelled => this.close_server_edit(window, cx),
        });
        (field, events)
    }

    pub(super) fn close_server_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.server_edit = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn next_server_field(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(edit) = self.server_edit.as_ref() else {
            return;
        };
        let on_name = edit.fields[NAME_FIELD].focus_handle(cx).is_focused(window);
        let next = &edit.fields[if on_name { ADDRESS_FIELD } else { NAME_FIELD }];
        window.focus(&next.focus_handle(cx), cx);
    }

    fn submit_server_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(edit) = self.server_edit.take() else {
            return;
        };
        let text = |index: usize| edit.fields[index].read(cx).text().trim().to_owned();
        let name = Some(text(NAME_FIELD)).filter(|name| !name.is_empty());
        let typed = text(ADDRESS_FIELD);
        self.close_server_edit(window, cx);
        let Some(uri) = network_uri(&typed) else {
            return self.show_toast(t!("server_edit.bad_address").to_string(), false, cx);
        };
        if uri == edit.original.uri {
            return cx.emit(BrowserEvent::ServerRenamed(SavedServer { uri, name }));
        }
        let pending = PendingEdit {
            replaces: edit.original.uri,
            name,
        };
        self.active_pane().clone().update(cx, |pane, cx| {
            pane.server_edit.pending = Some(pending);
            pane.connect_network(uri, None, cx)
        });
    }
}

impl BrowserView {
    fn server_edit_panel(&self, width: gpui::Pixels) -> Option<AnyElement> {
        let edit = self.server_edit.as_ref()?;
        let inputs = edit.fields.iter().enumerate().map(|(index, field)| {
            Self::dialog_input(t!(FIELD_LABELS[index]).to_string(), field.clone())
        });
        Some(
            div()
                .flex()
                .flex_col()
                .gap(px(TOOLBAR_GAP))
                .w(px(SETTINGS_PANEL_WIDTH).min(width - px(QUICK_LOOK_MARGIN * DOUBLE)))
                .p(px(SHEET_PANEL_PAD))
                .rounded(px(QUICK_LOOK_RADIUS))
                .bg(rgb(CARD))
                .border_1()
                .border_color(rgb(LINE_STRONG))
                .shadow_lg()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(
                    div()
                        .text_size(px(TEXT_EMPTY_TITLE))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(FG_BRIGHT))
                        .child(t!("server_edit.title").to_string()),
                )
                .children(inputs)
                .child(
                    div()
                        .text_color(rgb(FG_MUTED))
                        .child(t!("server_edit.hint").to_string()),
                )
                .into_any_element(),
        )
    }

    pub(super) fn network_dialogs(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        self.network_login_overlay(window, cx)
            .or_else(|| self.server_edit_overlay(window, cx))
    }

    fn server_edit_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let viewport = window.viewport_size();
        let panel = self.server_edit_panel(viewport.width)?;
        let focus = self.server_edit.as_ref()?.focus.clone();
        let view = cx.entity().downgrade();
        let on_dismiss: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_server_edit(window, cx));
        });
        let overlay = anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.0), px(0.0)))
            .child(ModalScrim {
                focus,
                context: SERVER_EDIT_CONTEXT,
                viewport,
                on_dismiss,
                panel,
            });
        Some(
            deferred(overlay)
                .with_priority(EDIT_PRIORITY)
                .into_any_element(),
        )
    }
}
