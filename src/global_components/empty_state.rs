use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{FG_BRIGHT, FG_MUTED};
use crate::core::theme::metrics::{EMPTY_GAP, EMPTY_ICON, TEXT_EMPTY_TITLE};
use crate::global_components::icon_glyph::IconGlyph;

#[derive(IntoElement)]
pub struct EmptyState {
    pub icon: Icon,
    pub title: SharedString,
    pub body: SharedString,
}

impl RenderOnce for EmptyState {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(EMPTY_GAP))
            .text_color(rgb(FG_MUTED))
            .child(IconGlyph::new(self.icon, FG_MUTED).size(EMPTY_ICON))
            .child(
                div()
                    .text_size(px(TEXT_EMPTY_TITLE))
                    .text_color(rgb(FG_BRIGHT))
                    .child(self.title),
            )
            .child(self.body)
    }
}
