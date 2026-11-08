use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::DIVIDER;
use crate::core::theme::metrics::{NEW_TAB_OFFSET, TABS_GAP, TABS_PAD_TOP, TABS_PAD_X};
use crate::features::shell::presentation::components::sheet_tab::SheetTab;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct TabStrip {
    pub title: SharedString,
}

impl RenderOnce for TabStrip {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(TABS_GAP))
            .pt(px(TABS_PAD_TOP))
            .px(px(TABS_PAD_X))
            .border_b_1()
            .border_color(rgb(DIVIDER))
            .child(SheetTab {
                index: 0,
                title: self.title,
                active: true,
            })
            .child(
                div()
                    .ml(px(NEW_TAB_OFFSET))
                    .child(IconButton::new("new-tab", Icon::Plus)),
            )
    }
}
