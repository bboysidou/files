use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px};

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::metrics::{TOOLBAR_GAP, TOOLBAR_PAD_X, TOOLBAR_PAD_Y};
use crate::features::browser::presentation::components::breadcrumbs::Breadcrumbs;
use crate::features::browser::presentation::components::filter_field::FilterField;
use crate::features::browser::presentation::components::view_switcher::ViewSwitcher;
use crate::features::browser::presentation::state::toolbar_model::ToolbarModel;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct Toolbar {
    pub model: ToolbarModel,
}

impl RenderOnce for Toolbar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let model = self.model;
        let filter_open = model.filter.is_some();
        let filter = model.filter.map(|filter| FilterField { model: filter });
        let filter_buttons = (!filter_open).then(|| {
            div()
                .flex()
                .gap(px(TOOLBAR_GAP))
                .child(IconButton::new("filter", Icon::Filter).on_click(model.on_filter))
                .child(IconButton::new("search", Icon::Search))
        });
        div()
            .flex()
            .flex_wrap()
            .flex_shrink_0()
            .items_center()
            .gap(px(TOOLBAR_GAP))
            .px(px(TOOLBAR_PAD_X))
            .py(px(TOOLBAR_PAD_Y))
            .child(
                IconButton::new("back", Icon::Back)
                    .disabled(!model.can_go_back)
                    .on_click(model.on_back),
            )
            .child(
                IconButton::new("forward", Icon::Forward)
                    .disabled(!model.can_go_forward)
                    .on_click(model.on_forward),
            )
            .child(
                IconButton::new("up", Icon::Up)
                    .disabled(!model.can_go_up)
                    .on_click(model.on_up),
            )
            .child(Breadcrumbs {
                trail: model.trail,
                on_crumb: model.on_crumb,
            })
            .children(filter)
            .children(filter_buttons)
            .child(ViewSwitcher {
                active: ViewMode::List,
            })
            .child(IconButton::new("preview", Icon::Panel))
            .child(IconButton::new("keys", Icon::Keys))
            .child(IconButton::new("settings", Icon::Sliders))
    }
}
