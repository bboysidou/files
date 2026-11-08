use std::time::SystemTime;

use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::helpers::files::extension_badge::extension_badge;
use crate::core::helpers::files::extension_color::extension_color;
use crate::core::helpers::time::relative_time::relative_time;
use crate::core::theme::colors::{FG, FG_MUTED, ROW_HOVER};
use crate::core::theme::metrics::{
    LIST_ROW_RADIUS, OVERVIEW_ITEM_GAP, OVERVIEW_ROW_HEIGHT, OVERVIEW_VALUE_WIDTH, TEXT_STATUS,
};
use crate::features::browser::presentation::components::file_badge::FileBadge;
use crate::features::overview::domain::entities::overview_report::RecentFile;
use crate::global_components::handlers::IndexHandler;

#[derive(IntoElement)]
pub struct RecentRow {
    pub index: usize,
    pub file: RecentFile,
    pub now: SystemTime,
    pub on_select: IndexHandler,
}

impl RenderOnce for RecentRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (index, on_select) = (self.index, self.on_select);
        let path = &self.file.path;
        let extension = path
            .extension()
            .map(|extension| extension.to_string_lossy().into_owned());
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let folder = path
            .parent()
            .map(|parent| parent.to_string_lossy().into_owned())
            .unwrap_or_default();
        div()
            .id(("recent", index))
            .flex()
            .items_center()
            .gap(px(OVERVIEW_ITEM_GAP))
            .h(px(OVERVIEW_ROW_HEIGHT))
            .rounded(px(LIST_ROW_RADIUS))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(ROW_HOVER)))
            .on_click(move |_, window, cx| on_select(index, window, cx))
            .child(FileBadge {
                text: extension_badge(extension.as_deref()).into(),
                color: extension_color(extension.as_deref()),
            })
            .child(div().flex_shrink_0().text_color(rgb(FG)).child(name))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .text_size(px(TEXT_STATUS))
                    .text_color(rgb(FG_MUTED))
                    .child(folder),
            )
            .child(
                div()
                    .w(px(OVERVIEW_VALUE_WIDTH))
                    .flex()
                    .justify_end()
                    .text_color(rgb(FG_MUTED))
                    .child(relative_time(self.file.modified, self.now)),
            )
    }
}
