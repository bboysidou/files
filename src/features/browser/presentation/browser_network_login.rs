use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Entity, Focusable, FontWeight, IntoElement,
    MouseButton, Pixels, Size, Subscription, Window, anchored, deferred, div, point, prelude::*,
    px, rgb,
};
use rust_i18n::t;

use crate::core::constants::network::{SCHEME_SEPARATOR, SMB_SCHEME};
use crate::core::theme::colors::{CARD, DANGER, FG_BRIGHT, FG_MUTED, LINE_STRONG};
use crate::core::theme::metrics::{
    QUICK_LOOK_MARGIN, QUICK_LOOK_RADIUS, SETTINGS_PANEL_WIDTH, SHEET_PANEL_PAD, TEXT_EMPTY_TITLE,
    TOOLBAR_GAP,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::key_bindings::LOGIN_CONTEXT;
use crate::features::browser::presentation::state::network_login_state::{
    NetworkLoginRequest, NetworkLoginState,
};
use crate::features::network::domain::entities::network_credentials::NetworkCredentials;
use crate::features::network::domain::usecases::network_uri_user::network_uri_user;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::modal_scrim::ModalScrim;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};

const LOGIN_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;
const USER_FIELD: usize = 0;
const PASSWORD_FIELD: usize = 1;
const DOMAIN_FIELD: usize = 2;
const FIELD_LABELS: [&str; 3] = ["login.user", "login.password", "login.domain"];

impl BrowserView {
    pub(super) fn open_network_login(
        &mut self,
        request: NetworkLoginRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let NetworkLoginRequest {
            pane,
            uri,
            rejected,
        } = request;
        let count = if uri.starts_with(SMB_SCHEME) { 3 } else { 2 };
        let (fields, events): (Vec<_>, Vec<_>) = (0..count)
            .map(|index| Self::login_field(index, window, cx))
            .unzip();
        let user = network_uri_user(&uri).unwrap_or_default();
        fields[USER_FIELD].update(cx, |field, cx| field.set_text(&user, 0..user.len(), cx));
        let first = if user.is_empty() {
            USER_FIELD
        } else {
            PASSWORD_FIELD
        };
        window.focus(&fields[first].focus_handle(cx), cx);
        self.network_login = Some(NetworkLoginState {
            pane,
            uri,
            rejected,
            fields,
            focus: cx.focus_handle(),
            _events: events,
        });
        cx.notify();
    }

    fn login_field(
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<TextField>, Subscription) {
        let field = cx.new(|cx| {
            let mut field = TextField::new(String::new(), cx);
            field.set_masked(index == PASSWORD_FIELD, cx);
            field
        });
        let events = cx.subscribe_in(&field, window, |this, _, event, window, cx| match event {
            TextFieldEvent::Changed => {}
            TextFieldEvent::Submitted => this.submit_network_login(window, cx),
            TextFieldEvent::Cancelled => this.close_network_login(window, cx),
        });
        (field, events)
    }

    pub(super) fn close_network_login(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.network_login = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn next_login_field(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(login) = self.network_login.as_ref() else {
            return;
        };
        let current = login
            .fields
            .iter()
            .position(|field| field.focus_handle(cx).is_focused(window))
            .unwrap_or_default();
        let next = &login.fields[(current + 1) % login.fields.len()];
        window.focus(&next.focus_handle(cx), cx);
    }

    fn submit_network_login(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(login) = self.network_login.take() else {
            return;
        };
        let text = |index: usize| {
            login
                .fields
                .get(index)
                .map(|field| field.read(cx).text().to_owned())
                .unwrap_or_default()
        };
        let credentials = NetworkCredentials {
            user: text(USER_FIELD),
            password: text(PASSWORD_FIELD),
            domain: text(DOMAIN_FIELD),
        };
        self.close_network_login(window, cx);
        let _ = login.pane.update(cx, |pane, cx| {
            pane.connect_network(login.uri, Some(credentials), cx)
        });
    }
}

impl BrowserView {
    fn login_panel(&self, viewport: Size<Pixels>) -> Option<AnyElement> {
        let login = self.network_login.as_ref()?;
        let host = login
            .uri
            .split_once(SCHEME_SEPARATOR)
            .map_or(login.uri.as_str(), |(_, rest)| rest)
            .trim_end_matches('/');
        let inputs = login.fields.iter().enumerate().map(|(index, field)| {
            Self::dialog_input(t!(FIELD_LABELS[index]).to_string(), field.clone())
        });
        let margin = px(QUICK_LOOK_MARGIN * DOUBLE);
        Some(
            div()
                .flex()
                .flex_col()
                .gap(px(TOOLBAR_GAP))
                .w(px(SETTINGS_PANEL_WIDTH).min(viewport.width - margin))
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
                        .child(t!("login.title", host = host).to_string()),
                )
                .when(login.rejected, |panel| {
                    panel.child(
                        div()
                            .text_color(rgb(DANGER))
                            .child(t!("login.rejected").to_string()),
                    )
                })
                .children(inputs)
                .child(
                    div()
                        .text_color(rgb(FG_MUTED))
                        .child(t!("login.hint").to_string()),
                )
                .into_any_element(),
        )
    }

    pub(super) fn network_login_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let viewport = window.viewport_size();
        let panel = self.login_panel(viewport)?;
        let focus = self.network_login.as_ref()?.focus.clone();
        let view = cx.entity().downgrade();
        let on_dismiss: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_network_login(window, cx));
        });
        let overlay = anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.0), px(0.0)))
            .child(ModalScrim {
                focus,
                context: LOGIN_CONTEXT,
                viewport,
                on_dismiss,
                panel,
            });
        Some(
            deferred(overlay)
                .with_priority(LOGIN_PRIORITY)
                .into_any_element(),
        )
    }
}
