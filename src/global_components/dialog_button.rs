use gpui::{
    App, FontWeight, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb,
};

use crate::core::enums::view::button_tone::ButtonTone;
use crate::core::theme::colors::{ACCENT, DANGER, FG, FRAME, LINE, LINE_STRONG, ON_ACCENT};
use crate::core::theme::metrics::{
    CONFIRM_BUTTON_HEIGHT, CONFIRM_BUTTON_PAD_X, CONFIRM_BUTTON_RADIUS,
};
use crate::global_components::handlers::ClickHandler;

#[derive(IntoElement)]
pub struct DialogButton {
    pub id: &'static str,
    pub label: SharedString,
    pub tone: ButtonTone,
    pub on_click: ClickHandler,
}

impl RenderOnce for DialogButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_click = self.on_click;
        let button = div()
            .id(self.id)
            .flex()
            .items_center()
            .h(px(CONFIRM_BUTTON_HEIGHT))
            .px(px(CONFIRM_BUTTON_PAD_X))
            .rounded(px(CONFIRM_BUTTON_RADIUS))
            .cursor_pointer()
            .on_click(move |_, window, cx| on_click(window, cx))
            .child(self.label);
        match self.tone {
            ButtonTone::Danger => button
                .bg(rgb(DANGER))
                .text_color(rgb(FRAME))
                .font_weight(FontWeight::SEMIBOLD),
            ButtonTone::Accent => button
                .bg(rgb(ACCENT))
                .text_color(rgb(ON_ACCENT))
                .font_weight(FontWeight::SEMIBOLD),
            ButtonTone::Plain => button
                .border_1()
                .border_color(rgb(LINE_STRONG))
                .text_color(rgb(FG))
                .hover(|style| style.bg(rgb(LINE))),
        }
    }
}
