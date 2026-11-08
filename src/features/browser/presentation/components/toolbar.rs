use gpui::{AnyElement, App, IntoElement, RenderOnce, Window, div, prelude::*, px};

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::metrics::{TOOLBAR_GAP, TOOLBAR_PAD_X, TOOLBAR_PAD_Y};
use crate::features::browser::presentation::components::breadcrumbs::Breadcrumbs;
use crate::features::browser::presentation::components::nav_buttons::NavButtons;
use crate::features::browser::presentation::components::path_field::PathField;
use crate::features::browser::presentation::components::toolbar_end::ToolbarEnd;
use crate::features::browser::presentation::components::toolbar_field::ToolbarField;
use crate::features::browser::presentation::state::filter_model::FilterModel;
use crate::features::browser::presentation::state::handlers::ViewHandler;
use crate::features::browser::presentation::state::toolbar_model::ToolbarModel;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_button::IconButton;

#[derive(IntoElement)]
pub struct Toolbar {
    pub model: ToolbarModel,
    pub preview_open: bool,
    pub on_preview: ClickHandler,
    pub view: ViewMode,
    pub on_view: ViewHandler,
    pub on_keys: ClickHandler,
}

impl Toolbar {
    fn field_area(
        search: Option<FilterModel>,
        filter: Option<FilterModel>,
        on_filter: ClickHandler,
        on_search: ClickHandler,
    ) -> AnyElement {
        if let Some(search) = search {
            return ToolbarField {
                icon: Icon::Search,
                model: search,
            }
            .into_any_element();
        }
        if let Some(filter) = filter {
            return ToolbarField {
                icon: Icon::Filter,
                model: filter,
            }
            .into_any_element();
        }
        div()
            .flex()
            .gap(px(TOOLBAR_GAP))
            .child(IconButton::new("filter", Icon::Filter).on_click(on_filter))
            .child(IconButton::new("search", Icon::Search).on_click(on_search))
            .into_any_element()
    }
}

impl RenderOnce for Toolbar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let model = self.model;
        let field = Self::field_area(model.search, model.filter, model.on_filter, model.on_search);
        div()
            .flex()
            .flex_wrap()
            .flex_shrink_0()
            .items_center()
            .gap(px(TOOLBAR_GAP))
            .px(px(TOOLBAR_PAD_X))
            .py(px(TOOLBAR_PAD_Y))
            .child(NavButtons {
                can_go_back: model.can_go_back,
                can_go_forward: model.can_go_forward,
                can_go_up: model.can_go_up,
                on_back: model.on_back,
                on_forward: model.on_forward,
                on_up: model.on_up,
            })
            .child(match model.path_field {
                Some(field) => PathField { field }.into_any_element(),
                None => Breadcrumbs {
                    trail: model.trail,
                    on_crumb: model.on_crumb,
                }
                .into_any_element(),
            })
            .child(field)
            .child(ToolbarEnd {
                view: self.view,
                on_view: self.on_view,
                preview_open: self.preview_open,
                on_preview: self.on_preview,
                on_keys: self.on_keys,
            })
    }
}
