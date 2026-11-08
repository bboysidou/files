use gpui::{
    AnyElement, App, IntoElement, MouseButton, RenderOnce, Window, div, prelude::*, px, rgb,
};

use crate::core::theme::colors::{LINE, LINE_STRONG, MENU_BG};
use crate::core::theme::metrics::{MENU_MIN_WIDTH, MENU_PAD, MENU_RADIUS, MENU_SEPARATOR_MARGIN};
use crate::global_components::handlers::IndexHandler;
use crate::global_components::menu::menu_entry::MenuEntry;
use crate::global_components::menu::menu_item::MenuItem;

#[derive(IntoElement)]
pub struct MenuPanel {
    pub entries: Vec<MenuEntry>,
    pub hot: usize,
    pub on_select: IndexHandler,
}

impl RenderOnce for MenuPanel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut children: Vec<AnyElement> = Vec::new();
        for (index, entry) in self.entries.into_iter().enumerate() {
            if entry.separated {
                children.push(
                    div()
                        .h(px(1.0))
                        .mx(px(MENU_SEPARATOR_MARGIN))
                        .my(px(MENU_SEPARATOR_MARGIN))
                        .bg(rgb(LINE))
                        .into_any_element(),
                );
            }
            children.push(
                MenuItem {
                    index,
                    label: entry.label,
                    icon: entry.icon,
                    shortcut: entry.shortcut,
                    enabled: entry.enabled,
                    danger: entry.danger,
                    hot: index == self.hot,
                    on_select: self.on_select.clone(),
                }
                .into_any_element(),
            );
        }
        div()
            .flex()
            .flex_col()
            .min_w(px(MENU_MIN_WIDTH))
            .p(px(MENU_PAD))
            .rounded(px(MENU_RADIUS))
            .bg(rgb(MENU_BG))
            .border_1()
            .border_color(rgb(LINE_STRONG))
            .shadow_lg()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .children(children)
    }
}
