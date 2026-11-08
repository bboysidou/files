use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{DANGER, FG, FG_BRIGHT, FG_DIM, FG_MUTED, LINE};
use crate::core::theme::metrics::{
    MENU_ITEM_GAP, MENU_ITEM_HEIGHT, MENU_ITEM_PAD_X, MENU_ITEM_RADIUS, MENU_SHORTCUT_PAD,
    TEXT_TINY,
};
use crate::global_components::handlers::IndexHandler;
use crate::global_components::icon_glyph::IconGlyph;

#[derive(IntoElement)]
pub struct MenuItem {
    pub index: usize,
    pub label: SharedString,
    pub icon: Icon,
    pub shortcut: SharedString,
    pub enabled: bool,
    pub danger: bool,
    pub hot: bool,
    pub on_select: IndexHandler,
}

impl RenderOnce for MenuItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = match (self.enabled, self.danger, self.hot) {
            (false, _, _) => FG_DIM,
            (true, true, _) => DANGER,
            (true, false, true) => FG_BRIGHT,
            (true, false, false) => FG,
        };
        let (index, on_select) = (self.index, self.on_select);
        let item = div()
            .id(("menu-item", index))
            .flex()
            .items_center()
            .gap(px(MENU_ITEM_GAP))
            .h(px(MENU_ITEM_HEIGHT))
            .px(px(MENU_ITEM_PAD_X))
            .rounded(px(MENU_ITEM_RADIUS))
            .text_color(rgb(color))
            .when(self.hot && self.enabled, |item| item.bg(rgb(LINE)))
            .child(IconGlyph::new(self.icon, color))
            .child(div().flex_1().child(self.label))
            .child(
                div()
                    .pl(px(MENU_SHORTCUT_PAD))
                    .text_size(px(TEXT_TINY))
                    .text_color(rgb(FG_MUTED))
                    .child(self.shortcut),
            );
        if !self.enabled {
            return item;
        }
        item.cursor_pointer()
            .hover(|style| style.bg(rgb(LINE)))
            .on_click(move |_, window, cx| on_select(index, window, cx))
    }
}
