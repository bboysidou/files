use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, FontWeight, IntoElement, MouseButton, Pixels,
    SharedString, Window, anchored, deferred, div, point, prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::enums::network::network_protocol::NetworkProtocol;
use crate::core::enums::network::server_field::ServerField;
use crate::core::enums::network::server_tab::ServerTab;
use crate::core::theme::colors::{CARD, DANGER, FG_BRIGHT, FG_MUTED, LINE_STRONG};
use crate::core::theme::metrics::{
    QUICK_LOOK_MARGIN, QUICK_LOOK_RADIUS, SERVER_PORT_WIDTH, SETTINGS_PANEL_WIDTH, SHEET_PANEL_PAD,
    TEXT_EMPTY_TITLE, TOOLBAR_GAP,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::key_bindings::SERVER_EDIT_CONTEXT;
use crate::features::browser::presentation::state::server_edit_state::ServerEditState;
use crate::features::network::domain::usecases::server_form_uri::server_form_uri;
use crate::global_components::choice_group::ChoiceGroup;
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::modal_scrim::ModalScrim;

const EDIT_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;
const PROTOCOL_CHOICE_ID: &str = "server-protocol";

impl BrowserView {
    fn server_input(edit: &ServerEditState, field: ServerField) -> AnyElement {
        Self::dialog_input(
            t!(field.label_key(edit.protocol)).to_string(),
            edit.fields[field.index()].clone(),
        )
    }

    fn server_protocols(edit: &ServerEditState, cx: &mut Context<Self>) -> AnyElement {
        let view = cx.entity().downgrade();
        let on_select: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = view.update(cx, |this, cx| this.select_server_protocol(index, cx));
        });
        ChoiceGroup {
            id: PROTOCOL_CHOICE_ID,
            labels: NetworkProtocol::ALL
                .iter()
                .map(|protocol| SharedString::from(protocol.badge()))
                .collect(),
            selected: NetworkProtocol::ALL
                .iter()
                .position(|protocol| *protocol == edit.protocol)
                .unwrap_or_default(),
            on_select,
        }
        .into_any_element()
    }

    fn server_status(edit: &ServerEditState, cx: &Context<Self>) -> AnyElement {
        let status = match (edit.error, server_form_uri(&Self::server_form(edit, cx))) {
            (Some(error), _) => div()
                .text_color(rgb(DANGER))
                .child(t!(error.label_key()).to_string()),
            (None, Ok(uri)) => div().text_color(rgb(FG_BRIGHT)).child(uri),
            (None, Err(_)) => div()
                .text_color(rgb(FG_MUTED))
                .child(t!("server_edit.preview_empty").to_string()),
        };
        status.into_any_element()
    }

    fn server_title(edit: &ServerEditState) -> AnyElement {
        let title = if edit.original.is_some() {
            "server_edit.title"
        } else {
            "server_edit.new_title"
        };
        div()
            .text_size(px(TEXT_EMPTY_TITLE))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(FG_BRIGHT))
            .child(t!(title).to_string())
            .into_any_element()
    }

    fn server_address_row(edit: &ServerEditState) -> AnyElement {
        div()
            .flex()
            .gap(px(TOOLBAR_GAP))
            .child(
                div()
                    .flex_1()
                    .child(Self::server_input(edit, ServerField::Host)),
            )
            .child(
                div()
                    .w(px(SERVER_PORT_WIDTH))
                    .child(Self::server_input(edit, ServerField::Port)),
            )
            .into_any_element()
    }

    fn server_form_view(edit: &ServerEditState, cx: &mut Context<Self>) -> Vec<AnyElement> {
        vec![
            Self::server_protocols(edit, cx),
            Self::server_address_row(edit),
            Self::server_input(edit, ServerField::User),
            Self::server_input(edit, ServerField::Path),
            Self::server_input(edit, ServerField::Name),
            Self::server_status(edit, cx),
            div()
                .text_color(rgb(FG_MUTED))
                .child(t!("server_edit.hint").to_string())
                .into_any_element(),
        ]
    }

    fn server_edit_panel(&self, width: Pixels, cx: &mut Context<Self>) -> Option<AnyElement> {
        let edit = self.server_edit.as_ref()?;
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
                .child(Self::server_title(edit))
                .children(Self::server_tabs(edit, cx))
                .children(match edit.tab {
                    ServerTab::Servers => Self::server_list_view(edit, cx),
                    ServerTab::New => Self::server_form_view(edit, cx),
                })
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
        let panel = self.server_edit_panel(viewport.width, cx)?;
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
