use std::rc::Rc;

use gpui::{App, Div, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::theme::focus_ring::focus_ring;
use crate::core::theme::colors::{CARD, FG_BRIGHT, FG_MUTED, FG_NAV};
use crate::core::theme::metrics::{
    CONTROL_RADIUS, ICON_BUTTON_SIZE, NAV_PAD_X, SERVER_ROW_HEIGHT, SERVER_SOURCE_WIDTH, TEXT_TINY,
    TOOLBAR_GAP,
};
use crate::features::network::domain::entities::server_choice::ServerChoice;
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::icon_button::IconButton;
use crate::global_components::protocol_badge::ProtocolBadge;

const ROW_ID: &str = "server-row";
const PIN_ID: &str = "server-pin";
const FORGET_ID: &str = "server-forget";

#[derive(IntoElement)]
pub struct ServerRow {
    pub index: usize,
    pub choice: ServerChoice,
    pub selected: bool,
    pub on_connect: IndexHandler,
    pub on_pin: IndexHandler,
    pub on_forget: Option<IndexHandler>,
}

fn bound(index: usize, handler: IndexHandler) -> ClickHandler {
    Rc::new(move |window, cx| handler(index, window, cx))
}

impl RenderOnce for ServerRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (index, on_connect) = (self.index, self.on_connect);
        div()
            .id((ROW_ID, index))
            .w_full()
            .flex()
            .items_center()
            .gap(px(TOOLBAR_GAP))
            .h(px(SERVER_ROW_HEIGHT))
            .px(px(NAV_PAD_X))
            .rounded(px(CONTROL_RADIUS))
            .border_1()
            .border_color(focus_ring(self.selected))
            .when(self.selected, |row| row.bg(rgb(CARD)))
            .cursor_pointer()
            .text_color(rgb(if self.selected { FG_BRIGHT } else { FG_NAV }))
            .hover(|style| style.bg(rgb(CARD)).text_color(rgb(FG_BRIGHT)))
            .on_click(move |_, window, cx| on_connect(index, window, cx))
            .child(div().flex_shrink_0().child(self.choice.label))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .text_color(rgb(FG_MUTED))
                    .children(self.choice.detail),
            )
            .children(
                self.choice
                    .protocol
                    .map(|protocol| ProtocolBadge { protocol }),
            )
            .child(
                div()
                    .w(px(SERVER_SOURCE_WIDTH))
                    .text_size(px(TEXT_TINY))
                    .text_color(rgb(FG_MUTED))
                    .child(t!(self.choice.source.label_key()).to_string()),
            )
            .child(row_actions(
                index,
                self.choice.pinned,
                self.on_pin,
                self.on_forget,
            ))
    }
}

fn row_actions(
    index: usize,
    pinned: bool,
    on_pin: IndexHandler,
    on_forget: Option<IndexHandler>,
) -> Div {
    let forget = match on_forget {
        Some(on_forget) => IconButton::new((FORGET_ID, index), Icon::Trash)
            .radius(CONTROL_RADIUS)
            .on_click(bound(index, on_forget))
            .into_any_element(),
        None => div().w(px(ICON_BUTTON_SIZE)).into_any_element(),
    };
    div()
        .flex()
        .flex_shrink_0()
        .items_center()
        .child(
            IconButton::new((PIN_ID, index), Icon::Pin)
                .radius(CONTROL_RADIUS)
                .active(pinned)
                .on_click(bound(index, on_pin)),
        )
        .child(forget)
}
