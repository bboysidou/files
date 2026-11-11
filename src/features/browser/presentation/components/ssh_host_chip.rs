use std::rc::Rc;

use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT, FG_MUTED, FG_NAV, LINE_STRONG};
use crate::core::theme::metrics::{CHOICE_GAP, CHOICE_PAD_X, CONTROL_RADIUS, TOOLBAR_GAP};
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::icon_button::IconButton;

const CHIP_ID: &str = "ssh-host";
const PIN_ID: &str = "ssh-pin";

#[derive(IntoElement)]
pub struct SshHostChip {
    pub index: usize,
    pub alias: SharedString,
    pub user: Option<SharedString>,
    pub selected: bool,
    pub pinned: bool,
    pub on_pick: IndexHandler,
    pub on_pin: IndexHandler,
}

impl RenderOnce for SshHostChip {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (index, on_pick, on_pin) = (self.index, self.on_pick, self.on_pin);
        let pin: ClickHandler = Rc::new(move |window, cx| on_pin(index, window, cx));
        let border = if self.selected { ACCENT } else { LINE_STRONG };
        div()
            .id((CHIP_ID, index))
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(CHOICE_GAP))
            .pl(px(CHOICE_PAD_X))
            .rounded(px(CONTROL_RADIUS))
            .border_1()
            .border_color(rgb(border))
            .cursor_pointer()
            .text_color(rgb(if self.selected { FG_BRIGHT } else { FG_NAV }))
            .hover(|style| style.bg(rgb(CARD)).text_color(rgb(FG_BRIGHT)))
            .on_click(move |_, window, cx| on_pick(index, window, cx))
            .child(self.alias)
            .when_some(self.user, |chip, user| {
                chip.child(
                    div()
                        .ml(px(TOOLBAR_GAP))
                        .text_color(rgb(FG_MUTED))
                        .child(user),
                )
            })
            .child(
                IconButton::new((PIN_ID, index), Icon::Pin)
                    .radius(CONTROL_RADIUS)
                    .active(self.pinned)
                    .on_click(pin),
            )
    }
}
