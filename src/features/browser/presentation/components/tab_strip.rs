use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::DIVIDER;
use crate::core::theme::metrics::{NEW_TAB_OFFSET, TABS_GAP, TABS_PAD_TOP, TABS_PAD_X};
use crate::features::browser::presentation::components::sheet_tab::SheetTab;
use crate::features::browser::presentation::state::tab_strip_model::TabStripModel;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct TabStrip {
    pub model: TabStripModel,
}

impl RenderOnce for TabStrip {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let model = self.model;
        let tabs = model
            .titles
            .into_iter()
            .enumerate()
            .map(|(index, title)| SheetTab {
                index,
                title,
                active: index == model.active,
                on_select: model.on_select.clone(),
                on_close: model.on_close.clone(),
                on_move: model.on_move.clone(),
            });
        div()
            .id("tab-strip")
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(TABS_GAP))
            .pt(px(TABS_PAD_TOP))
            .px(px(TABS_PAD_X))
            .overflow_x_scroll()
            .border_b_1()
            .border_color(rgb(DIVIDER))
            .children(tabs)
            .child(
                div()
                    .flex_shrink_0()
                    .ml(px(NEW_TAB_OFFSET))
                    .child(IconButton::new("new-tab", Icon::Plus).on_click(model.on_new)),
            )
    }
}
