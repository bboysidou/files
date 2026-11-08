use gpui::{
    AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px, rgb,
};

use crate::core::theme::colors::SHEET;
use crate::core::theme::metrics::{BODY_PAD_BOTTOM, BODY_PAD_X, SHEET_RADIUS};
use crate::features::browser::presentation::components::status_bar::StatusBar;
use crate::features::browser::presentation::components::tab_strip::TabStrip;
use crate::features::browser::presentation::components::toolbar::Toolbar;
use crate::features::browser::presentation::state::toolbar_model::ToolbarModel;

#[derive(IntoElement)]
pub struct Sheet {
    pub title: SharedString,
    pub toolbar: ToolbarModel,
    pub body: AnyElement,
    pub item_count: usize,
    pub selected_count: usize,
}

impl RenderOnce for Sheet {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let location = self.toolbar.trail.display_path();
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .h_full()
            .overflow_hidden()
            .rounded(px(SHEET_RADIUS))
            .bg(rgb(SHEET))
            .child(TabStrip { title: self.title })
            .child(Toolbar {
                model: self.toolbar,
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h_0()
                    .px(px(BODY_PAD_X))
                    .pb(px(BODY_PAD_BOTTOM))
                    .child(self.body),
            )
            .child(StatusBar {
                location: location.into(),
                item_count: self.item_count,
                selected_count: self.selected_count,
            })
    }
}
