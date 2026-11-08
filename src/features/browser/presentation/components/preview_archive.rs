use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{FG, FG_MUTED};
use crate::core::theme::metrics::{FOLDER_PREVIEW_GAP, FOLDER_PREVIEW_PAD_X, FOLDER_PREVIEW_PAD_Y};
use crate::features::preview::domain::entities::archive_member::ArchiveMember;
use crate::global_components::icon_glyph::IconGlyph;

#[derive(IntoElement)]
pub struct PreviewArchive {
    pub members: Vec<ArchiveMember>,
    pub total: usize,
    pub capped: bool,
}

impl RenderOnce for PreviewArchive {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let hidden = self.total.saturating_sub(self.members.len());
        let rows = self.members.into_iter().map(|member| {
            let icon = if member.is_dir {
                Icon::Folder
            } else {
                Icon::Document
            };
            div()
                .flex()
                .items_center()
                .gap(px(FOLDER_PREVIEW_GAP))
                .px(px(FOLDER_PREVIEW_PAD_X))
                .py(px(FOLDER_PREVIEW_PAD_Y))
                .child(IconGlyph::new(icon, FG_MUTED))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .overflow_hidden()
                        .text_ellipsis()
                        .whitespace_nowrap()
                        .text_color(rgb(FG))
                        .child(member.path),
                )
        });
        let more = match (hidden, self.capped) {
            (0, _) => None,
            (count, false) => Some(t!("ql.more", count = count).to_string()),
            (count, true) => Some(t!("ql.more_capped", count = count).to_string()),
        };
        div()
            .id("preview-archive")
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .children(rows)
            .children(more.map(|text| {
                div()
                    .px(px(FOLDER_PREVIEW_PAD_X))
                    .py(px(FOLDER_PREVIEW_PAD_Y))
                    .text_color(rgb(FG_MUTED))
                    .child(text)
            }))
    }
}
