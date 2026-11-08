use gpui::ScrollHandle;
use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::helpers::format::size_text::size_text;
use crate::core::theme::colors::{FG, FG_MUTED};
use crate::core::theme::metrics::{
    FOLDER_PREVIEW_GAP, FOLDER_PREVIEW_PAD_X, FOLDER_PREVIEW_PAD_Y, TEXT_STATUS,
};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::components::entry_icon::EntryIcon;

#[derive(IntoElement)]
pub struct PreviewFolder {
    pub scroll: ScrollHandle,
    pub entries: Vec<FileEntry>,
    pub total: usize,
}

impl PreviewFolder {
    fn detail(entry: &FileEntry) -> String {
        size_text(entry.is_dir, entry.child_count, entry.size)
    }
}

impl RenderOnce for PreviewFolder {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let hidden = self.total.saturating_sub(self.entries.len());
        let rows = self.entries.into_iter().map(|entry| {
            let detail = Self::detail(&entry);
            div()
                .flex()
                .items_center()
                .gap(px(FOLDER_PREVIEW_GAP))
                .px(px(FOLDER_PREVIEW_PAD_X))
                .py(px(FOLDER_PREVIEW_PAD_Y))
                .child(EntryIcon {
                    entry: entry.clone(),
                })
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .overflow_hidden()
                        .text_ellipsis()
                        .whitespace_nowrap()
                        .text_color(rgb(FG))
                        .child(entry.name.clone()),
                )
                .child(
                    div()
                        .flex_shrink_0()
                        .text_size(px(TEXT_STATUS))
                        .text_color(rgb(FG_MUTED))
                        .child(detail),
                )
        });
        let list = div()
            .id("preview-folder")
            .track_scroll(&self.scroll)
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .children(rows);
        if hidden == 0 {
            return list;
        }
        list.child(
            div()
                .px(px(FOLDER_PREVIEW_PAD_X))
                .py(px(FOLDER_PREVIEW_PAD_Y))
                .text_color(rgb(FG_MUTED))
                .child(t!("ql.more", count = hidden).to_string()),
        )
    }
}
