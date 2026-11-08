use gpui::{
    AnyElement, App, Entity, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
    rgb,
};

use crate::core::enums::view::button_tone::ButtonTone;
use crate::core::theme::colors::{ACCENT, CARD, DIVIDER, FG_BRIGHT};
use crate::core::theme::metrics::{
    CONFIRM_GAP, CONTROL_RADIUS, CRUMBS_HEIGHT, FILTER_FIELD_PAD_X, TOOLBAR_PAD_X, TOOLBAR_PAD_Y,
};
use crate::global_components::dialog_button::DialogButton;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::text_field::text_field_view::TextField;

#[derive(IntoElement)]
pub struct PickerFooter {
    pub name_field: Option<Entity<TextField>>,
    pub cancel_label: SharedString,
    pub accept_label: SharedString,
    pub on_cancel: ClickHandler,
    pub on_accept: ClickHandler,
}

impl RenderOnce for PickerFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let name: AnyElement = match self.name_field {
            Some(field) => div()
                .flex()
                .flex_1()
                .items_center()
                .h(px(CRUMBS_HEIGHT))
                .px(px(FILTER_FIELD_PAD_X))
                .rounded(px(CONTROL_RADIUS))
                .bg(rgb(CARD))
                .border_1()
                .border_color(rgb(ACCENT))
                .text_color(rgb(FG_BRIGHT))
                .child(field)
                .into_any_element(),
            None => div().flex_1().into_any_element(),
        };
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(CONFIRM_GAP))
            .px(px(TOOLBAR_PAD_X))
            .py(px(TOOLBAR_PAD_Y))
            .border_t_1()
            .border_color(rgb(DIVIDER))
            .child(name)
            .child(DialogButton {
                id: "picker-cancel",
                label: self.cancel_label,
                tone: ButtonTone::Plain,
                on_click: self.on_cancel,
            })
            .child(DialogButton {
                id: "picker-accept",
                label: self.accept_label,
                tone: ButtonTone::Accent,
                on_click: self.on_accept,
            })
    }
}
