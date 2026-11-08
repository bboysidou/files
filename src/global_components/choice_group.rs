use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::theme::colors::{ACCENT, CARD, FG_BRIGHT, FG_NAV, LINE_STRONG, ON_ACCENT};
use crate::core::theme::metrics::{
    CHOICE_GAP, CHOICE_PAD_X, CHOICE_PAD_Y, CHOICE_RADIUS, CONTROL_RADIUS,
};
use crate::global_components::handlers::IndexHandler;

#[derive(IntoElement)]
pub struct ChoiceGroup {
    pub id: &'static str,
    pub labels: Vec<SharedString>,
    pub selected: usize,
    pub on_select: IndexHandler,
}

impl RenderOnce for ChoiceGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (id, selected, on_select) = (self.id, self.selected, self.on_select);
        div()
            .flex()
            .flex_shrink_0()
            .gap(px(CHOICE_GAP))
            .p(px(CHOICE_GAP))
            .rounded(px(CONTROL_RADIUS))
            .border_1()
            .border_color(rgb(LINE_STRONG))
            .children(
                self.labels
                    .into_iter()
                    .enumerate()
                    .map(move |(index, label)| {
                        let on_select = on_select.clone();
                        div()
                            .id((id, index))
                            .px(px(CHOICE_PAD_X))
                            .py(px(CHOICE_PAD_Y))
                            .rounded(px(CHOICE_RADIUS))
                            .cursor_pointer()
                            .map(|choice| {
                                if index == selected {
                                    choice.bg(rgb(ACCENT)).text_color(rgb(ON_ACCENT))
                                } else {
                                    choice.text_color(rgb(FG_NAV)).hover(|style| {
                                        style.bg(rgb(CARD)).text_color(rgb(FG_BRIGHT))
                                    })
                                }
                            })
                            .on_click(move |_, window, cx| on_select(index, window, cx))
                            .child(label)
                    }),
            )
    }
}
