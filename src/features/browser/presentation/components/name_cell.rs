use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::files::extension_badge::extension_badge;
use crate::core::theme::colors::{ACCENT, FG, FG_BRIGHT};
use crate::core::theme::metrics::{FOLDER_ICON, LIST_CELL_PAD_X, LIST_NAME_GAP, TEXT_BASE};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::components::file_badge::FileBadge;
use crate::global_components::icon_glyph::IconGlyph;

#[derive(IntoElement)]
pub struct NameCell {
    pub entry: FileEntry,
}

impl RenderOnce for NameCell {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let cell = div()
            .flex()
            .flex_1()
            .min_w_0()
            .items_center()
            .gap(px(LIST_NAME_GAP))
            .px(px(LIST_CELL_PAD_X))
            .text_size(px(TEXT_BASE));
        let label = div()
            .min_w_0()
            .overflow_hidden()
            .whitespace_nowrap()
            .text_ellipsis()
            .child(self.entry.name);
        if self.entry.is_dir {
            return cell
                .text_color(rgb(FG_BRIGHT))
                .child(IconGlyph::new(Icon::Folder, ACCENT).size(FOLDER_ICON))
                .child(label);
        }
        let badge = FileBadge {
            text: extension_badge(self.entry.extension.as_deref()).into(),
            color: self.entry.kind.color(),
        };
        cell.text_color(rgb(FG)).child(badge).child(label)
    }
}
