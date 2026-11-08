use gpui::{App, ElementId, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{FG_BRIGHT, FG_DIM, FG_NAV, LINE, LINE_STRONG};
use crate::core::theme::metrics::ICON_BUTTON_SIZE;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_glyph::IconGlyph;

const GROUP: &str = "icon-button";

#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: Icon,
    radius: Option<f32>,
    active: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: Icon) -> Self {
        Self {
            id: id.into(),
            icon,
            radius: None,
            active: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, handler: ClickHandler) -> Self {
        self.on_click = Some(handler);
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = match (self.disabled, self.active) {
            (true, _) => FG_DIM,
            (false, true) => FG_BRIGHT,
            (false, false) => FG_NAV,
        };
        let shell = div()
            .id(self.id)
            .group(GROUP)
            .size(px(ICON_BUTTON_SIZE))
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center();
        let shell = match self.radius {
            Some(radius) => shell.rounded(px(radius)),
            None => shell.rounded_full(),
        };
        let shell = if self.active {
            shell.bg(rgb(LINE_STRONG))
        } else {
            shell
        };
        let glyph = IconGlyph::new(self.icon, color);
        if self.disabled {
            return shell.child(glyph);
        }
        let shell = shell.cursor_pointer().hover(|style| style.bg(rgb(LINE)));
        let shell = match self.on_click {
            Some(handler) => shell.on_click(move |_, window, cx| handler(window, cx)),
            None => shell,
        };
        shell.child(glyph.group_hover(GROUP, FG_BRIGHT))
    }
}
