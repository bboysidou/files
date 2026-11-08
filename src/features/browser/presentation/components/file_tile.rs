use gpui::{
    AnyElement, App, IntoElement, ObjectFit, RenderOnce, Window, div, img, prelude::*, px, rgb,
    rgba,
};

use crate::core::constants::preview::IMAGE_EXTENSIONS;
use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{ACCENT, CARD, FG, ROW_HOVER, SELECTION_ALPHA, TRANSPARENT};
use crate::core::theme::metrics::{
    CARD_RADIUS, CUT_OPACITY, TEXT_STATUS, TILE_FOLDER_ICON, TILE_INNER_GAP, TILE_NAME_HEIGHT,
    TILE_PAD, TILE_THUMB_RADIUS,
};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::components::entry_icon::EntryIcon;
use crate::global_components::handlers::IndexClickHandler;
use crate::global_components::icon_glyph::IconGlyph;

#[derive(IntoElement)]
pub struct FileTile {
    pub index: usize,
    pub entry: FileEntry,
    pub marked: bool,
    pub cursor: bool,
    pub cut: bool,
    pub width: f32,
    pub thumb_height: f32,
    pub on_click: IndexClickHandler,
}

impl FileTile {
    fn thumb_content(entry: &FileEntry) -> AnyElement {
        let is_image = entry.extension.as_deref().is_some_and(|extension| {
            IMAGE_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str())
        });
        if entry.is_dir {
            IconGlyph::new(Icon::Folder, ACCENT)
                .size(TILE_FOLDER_ICON)
                .into_any_element()
        } else if is_image {
            img(entry.path.clone())
                .object_fit(ObjectFit::Cover)
                .size_full()
                .rounded(px(TILE_THUMB_RADIUS))
                .with_fallback({
                    let entry = entry.clone();
                    move || {
                        div()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(EntryIcon {
                                entry: entry.clone(),
                            })
                            .into_any_element()
                    }
                })
                .into_any_element()
        } else {
            EntryIcon {
                entry: entry.clone(),
            }
            .into_any_element()
        }
    }
}

impl RenderOnce for FileTile {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (index, on_click) = (self.index, self.on_click);
        let thumb = div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h(px(self.thumb_height))
            .overflow_hidden()
            .rounded(px(TILE_THUMB_RADIUS))
            .bg(rgb(CARD))
            .child(Self::thumb_content(&self.entry));
        let name = div()
            .w_full()
            .h(px(TILE_NAME_HEIGHT))
            .flex()
            .justify_center()
            .overflow_hidden()
            .whitespace_nowrap()
            .text_ellipsis()
            .text_size(px(TEXT_STATUS))
            .text_color(rgb(FG))
            .child(self.entry.name.clone());
        let tile = div()
            .id(("tile", index))
            .flex()
            .flex_col()
            .flex_shrink_0()
            .items_center()
            .gap(px(TILE_INNER_GAP))
            .w(px(self.width))
            .p(px(TILE_PAD))
            .rounded(px(CARD_RADIUS))
            .border_1()
            .border_color(if self.cursor {
                rgb(ACCENT)
            } else {
                rgba(TRANSPARENT)
            })
            .when(self.cut, |tile| tile.opacity(CUT_OPACITY))
            .on_click(move |event, window, cx| on_click(index, event, window, cx))
            .child(thumb)
            .child(name);
        if self.marked {
            tile.bg(rgba((ACCENT << 8) | SELECTION_ALPHA))
        } else {
            tile.hover(|style| style.bg(rgb(ROW_HOVER)))
        }
    }
}
