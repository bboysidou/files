use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::metrics::TOOLBAR_GAP;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct NavButtons {
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub can_go_up: bool,
    pub on_back: ClickHandler,
    pub on_forward: ClickHandler,
    pub on_up: ClickHandler,
}

impl RenderOnce for NavButtons {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .gap(px(TOOLBAR_GAP))
            .child(
                IconButton::new("back", Icon::Back)
                    .disabled(!self.can_go_back)
                    .on_click(self.on_back),
            )
            .child(
                IconButton::new("forward", Icon::Forward)
                    .disabled(!self.can_go_forward)
                    .on_click(self.on_forward),
            )
            .child(
                IconButton::new("up", Icon::Up)
                    .disabled(!self.can_go_up)
                    .on_click(self.on_up),
            )
    }
}
