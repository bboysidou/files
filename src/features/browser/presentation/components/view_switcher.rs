use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::colors::{CARD, LINE};
use crate::core::theme::metrics::{SEGMENTED_GAP, SEGMENTED_PAD};
use crate::features::browser::presentation::state::handlers::ViewHandler;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct ViewSwitcher {
    pub active: ViewMode,
    pub on_select: ViewHandler,
}

impl RenderOnce for ViewSwitcher {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let buttons = ViewMode::ALL.iter().enumerate().map(|(index, mode)| {
            let (mode, on_select) = (*mode, self.on_select.clone());
            IconButton::new(("view", index), mode.icon())
                .active(mode == self.active)
                .disabled(!mode.is_available())
                .on_click(std::rc::Rc::new(move |window, cx| {
                    on_select(mode, window, cx)
                }))
        });
        div()
            .flex()
            .flex_shrink_0()
            .gap(px(SEGMENTED_GAP))
            .p(px(SEGMENTED_PAD))
            .rounded_full()
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(LINE))
            .children(buttons)
    }
}
