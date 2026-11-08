use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px};

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::metrics::{TOOLBAR_GAP, TOOLBAR_PAD_X, TOOLBAR_PAD_Y};
use crate::features::places::domain::entities::crumb_trail::CrumbTrail;
use crate::features::shell::presentation::components::breadcrumbs::Breadcrumbs;
use crate::features::shell::presentation::components::view_switcher::ViewSwitcher;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct Toolbar {
    pub trail: CrumbTrail,
}

impl RenderOnce for Toolbar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let at_root = self.trail.segments.is_empty();
        div()
            .flex()
            .flex_wrap()
            .flex_shrink_0()
            .items_center()
            .gap(px(TOOLBAR_GAP))
            .px(px(TOOLBAR_PAD_X))
            .py(px(TOOLBAR_PAD_Y))
            .child(IconButton::new("back", Icon::Back).disabled(true))
            .child(IconButton::new("forward", Icon::Forward).disabled(true))
            .child(IconButton::new("up", Icon::Up).disabled(at_root))
            .child(Breadcrumbs { trail: self.trail })
            .child(IconButton::new("filter", Icon::Filter))
            .child(IconButton::new("search", Icon::Search))
            .child(ViewSwitcher {
                active: ViewMode::List,
            })
            .child(IconButton::new("preview", Icon::Panel))
            .child(IconButton::new("keys", Icon::Keys))
            .child(IconButton::new("settings", Icon::Sliders))
    }
}
