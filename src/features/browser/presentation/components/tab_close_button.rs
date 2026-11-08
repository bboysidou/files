use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{FG_BRIGHT, FG_MUTED, LINE_STRONG};
use crate::core::theme::metrics::{ICON_SIZE_SMALL, TAB_CLOSE_RADIUS, TAB_CLOSE_SIZE};
use crate::global_components::handlers::IndexHandler;
use crate::global_components::icon_glyph::IconGlyph;

const GROUP: &str = "tab-close";

#[derive(IntoElement)]
pub struct TabCloseButton {
    pub index: usize,
    pub on_close: IndexHandler,
}

impl RenderOnce for TabCloseButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (index, on_close) = (self.index, self.on_close);
        div()
            .id(("tab-close", index))
            .group(GROUP)
            .size(px(TAB_CLOSE_SIZE))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(TAB_CLOSE_RADIUS))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(LINE_STRONG)))
            .on_click(move |_, window, cx| {
                cx.stop_propagation();
                on_close(index, window, cx);
            })
            .child(
                IconGlyph::new(Icon::Close, FG_MUTED)
                    .size(ICON_SIZE_SMALL)
                    .group_hover(GROUP, FG_BRIGHT),
            )
    }
}
