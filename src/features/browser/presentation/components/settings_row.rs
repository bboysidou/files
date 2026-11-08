use gpui::{
    AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb,
};

use crate::core::theme::colors::{FG_NAV, LINE};
use crate::core::theme::metrics::{SETTINGS_ROW_HEIGHT, TOOLBAR_GAP};

#[derive(IntoElement)]
pub struct SettingsRow {
    pub label: SharedString,
    pub control: AnyElement,
}

impl RenderOnce for SettingsRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_between()
            .gap(px(TOOLBAR_GAP))
            .h(px(SETTINGS_ROW_HEIGHT))
            .border_b_1()
            .border_color(rgb(LINE))
            .text_color(rgb(FG_NAV))
            .child(self.label)
            .child(self.control)
    }
}
