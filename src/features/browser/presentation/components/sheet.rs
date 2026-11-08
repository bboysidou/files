use gpui::{AnyElement, App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::colors::SHEET;
use crate::core::theme::metrics::{BODY_GAP, BODY_PAD_BOTTOM, BODY_PAD_X, SHEET_RADIUS};
use crate::features::browser::presentation::components::status_bar::StatusBar;
use crate::features::browser::presentation::components::tab_strip::TabStrip;
use crate::features::browser::presentation::components::toolbar::Toolbar;
use crate::features::browser::presentation::state::handlers::ViewHandler;
use crate::features::browser::presentation::state::tab_strip_model::TabStripModel;
use crate::features::browser::presentation::state::toolbar_model::ToolbarModel;
use crate::global_components::handlers::ClickHandler;

#[derive(IntoElement)]
pub struct Sheet {
    pub tabs: TabStripModel,
    pub toolbar: ToolbarModel,
    pub body: AnyElement,
    pub item_count: usize,
    pub selected_count: usize,
    pub side: Option<AnyElement>,
    pub preview_open: bool,
    pub on_preview: ClickHandler,
    pub view: ViewMode,
    pub on_view: ViewHandler,
    pub on_keys: ClickHandler,
    pub on_settings: ClickHandler,
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
            .child(TabStrip { model: self.tabs })
            .child(Toolbar {
                model: self.toolbar,
                preview_open: self.preview_open,
                on_preview: self.on_preview,
                view: self.view,
                on_view: self.on_view,
                on_keys: self.on_keys,
                on_settings: self.on_settings,
            })
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .gap(px(BODY_GAP))
                    .px(px(BODY_PAD_X))
                    .pb(px(BODY_PAD_BOTTOM))
                    .child(div().flex().flex_col().flex_1().min_w_0().child(self.body))
                    .children(self.side),
            )
            .child(StatusBar {
                location: location.into(),
                item_count: self.item_count,
                selected_count: self.selected_count,
            })
    }
}
