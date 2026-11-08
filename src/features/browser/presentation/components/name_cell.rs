use gpui::{AnyElement, App, Entity, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::files::extension_badge::extension_badge;
use crate::core::helpers::files::extension_color::extension_color;
use crate::core::theme::colors::{ACCENT, FG, FG_BRIGHT, FG_MUTED};
use crate::core::theme::metrics::{
    CUT_OPACITY, FOLDER_ICON, LIST_CELL_PAD_X, LIST_NAME_GAP, TEXT_BASE, TEXT_STATUS,
};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::components::file_badge::FileBadge;
use crate::features::browser::presentation::components::rename_box::RenameBox;
use crate::global_components::icon_glyph::IconGlyph;
use crate::global_components::tag_dots::TagDots;
use crate::global_components::text_field::text_field_view::TextField;

#[derive(IntoElement)]
pub struct NameCell {
    pub entry: FileEntry,
    pub cut: bool,
    pub rename: Option<Entity<TextField>>,
    pub context: Option<String>,
}

impl RenderOnce for NameCell {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self.context.clone().map(|context| {
            div()
                .min_w_0()
                .overflow_hidden()
                .whitespace_nowrap()
                .text_ellipsis()
                .text_size(px(TEXT_STATUS))
                .text_color(rgb(FG_MUTED))
                .child(context)
        });
        let cell = div()
            .flex()
            .flex_1()
            .min_w_0()
            .items_center()
            .gap(px(LIST_NAME_GAP))
            .px(px(LIST_CELL_PAD_X))
            .text_size(px(TEXT_BASE));
        let dots = (!self.entry.tags.is_empty()).then(|| TagDots {
            tags: self.entry.tags.clone(),
        });
        let label: AnyElement = match self.rename {
            Some(field) => RenameBox { field }.into_any_element(),
            None => div()
                .min_w_0()
                .overflow_hidden()
                .whitespace_nowrap()
                .text_ellipsis()
                .when(self.cut, |label| label.opacity(CUT_OPACITY))
                .child(self.entry.name)
                .into_any_element(),
        };
        if self.entry.is_dir {
            return cell
                .text_color(rgb(FG_BRIGHT))
                .child(IconGlyph::new(Icon::Folder, ACCENT).size(FOLDER_ICON))
                .child(label)
                .children(dots)
                .children(context);
        }
        let badge = FileBadge {
            text: extension_badge(self.entry.extension.as_deref()).into(),
            color: extension_color(self.entry.extension.as_deref()),
        };
        cell.text_color(rgb(FG))
            .child(badge)
            .child(label)
            .children(dots)
            .children(context)
    }
}
