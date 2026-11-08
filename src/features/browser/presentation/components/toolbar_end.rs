use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px};

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::metrics::TOOLBAR_GAP;
use crate::features::browser::presentation::components::view_switcher::ViewSwitcher;
use crate::features::browser::presentation::state::handlers::ViewHandler;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct ToolbarEnd {
    pub view: ViewMode,
    pub on_view: ViewHandler,
    pub preview_open: bool,
    pub on_preview: ClickHandler,
    pub on_keys: ClickHandler,
}

impl RenderOnce for ToolbarEnd {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(TOOLBAR_GAP))
            .child(ViewSwitcher {
                active: self.view,
                on_select: self.on_view,
            })
            .child(
                IconButton::new("preview", Icon::Panel)
                    .active(self.preview_open)
                    .on_click(self.on_preview),
            )
            .child(IconButton::new("keys", Icon::Keys).on_click(self.on_keys))
            .child(IconButton::new("settings", Icon::Sliders))
    }
}
