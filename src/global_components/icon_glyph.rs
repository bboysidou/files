use gpui::{App, IntoElement, RenderOnce, SharedString, Window, prelude::*, px, rgb, svg};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::metrics::ICON_SIZE;

#[derive(IntoElement)]
pub struct IconGlyph {
    icon: Icon,
    color: u32,
    size: f32,
    hover: Option<(SharedString, u32)>,
}

impl IconGlyph {
    pub fn new(icon: Icon, color: u32) -> Self {
        Self {
            icon,
            color,
            size: ICON_SIZE,
            hover: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn group_hover(mut self, group: impl Into<SharedString>, color: u32) -> Self {
        self.hover = Some((group.into(), color));
        self
    }
}

impl RenderOnce for IconGlyph {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let glyph = svg()
            .path(self.icon.path())
            .size(px(self.size))
            .flex_shrink_0()
            .text_color(rgb(self.color));
        match self.hover {
            Some((group, color)) => {
                glyph.group_hover(group, move |style| style.text_color(rgb(color)))
            }
            None => glyph,
        }
    }
}
