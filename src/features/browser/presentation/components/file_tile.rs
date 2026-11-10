use std::path::PathBuf;

use gpui::{
    AnyElement, App, IntoElement, MouseButton, ObjectFit, RenderOnce, Window, div, img, prelude::*,
    px, rgb, rgba,
};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{
    ACCENT, CARD, FG, FG_MUTED, ROW_HOVER, SELECTION_ALPHA, TRANSPARENT,
};
use crate::core::theme::metrics::ENTRY_TAG_GAP;
use crate::core::theme::metrics::{
    CARD_RADIUS, CUT_OPACITY, TEXT_STATUS, TILE_FOLDER_ICON, TILE_INNER_GAP, TILE_NAME_HEIGHT,
    TILE_PAD, TILE_THUMB_RADIUS,
};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::components::entry_icon::EntryIcon;
use crate::global_components::drag::drag_slot::DragSlot;
use crate::global_components::handlers::{IndexClickHandler, IndexPointHandler};
use crate::global_components::icon_glyph::IconGlyph;
use crate::global_components::tag_dots::TagDots;

#[derive(IntoElement)]
pub struct FileTile {
    pub index: usize,
    pub drag: DragSlot,
    pub entry: FileEntry,
    pub thumbnail: Option<PathBuf>,
    pub marked: bool,
    pub cursor: bool,
    pub focused: bool,
    pub cut: bool,
    pub width: f32,
    pub thumb_height: f32,
    pub on_click: IndexClickHandler,
    pub on_right: IndexPointHandler,
}

impl FileTile {
    fn thumb_content(entry: &FileEntry, thumbnail: Option<PathBuf>) -> AnyElement {
        if entry.is_dir {
            IconGlyph::new(Icon::Folder, ACCENT)
                .size(TILE_FOLDER_ICON)
                .into_any_element()
        } else if let Some(thumbnail) = thumbnail {
            img(thumbnail)
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

impl FileTile {
    fn thumb(&self) -> gpui::Div {
        div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h(px(self.thumb_height))
            .overflow_hidden()
            .rounded(px(TILE_THUMB_RADIUS))
            .bg(rgb(CARD))
            .child(Self::thumb_content(&self.entry, self.thumbnail.clone()))
    }
}

impl RenderOnce for FileTile {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let thumb = self.thumb();
        let (index, on_click, on_right) = (self.index, self.on_click, self.on_right);
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
            .gap(px(ENTRY_TAG_GAP))
            .items_center()
            .when(!self.entry.tags.is_empty(), |name| {
                name.child(TagDots {
                    tags: self.entry.tags.clone(),
                })
            })
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
            .border_color(match (self.cursor, self.focused) {
                (true, true) => rgb(ACCENT),
                (true, false) => rgb(FG_MUTED),
                (false, _) => rgba(TRANSPARENT),
            })
            .when(self.cut, |tile| tile.opacity(CUT_OPACITY))
            .on_click(move |event, window, cx| on_click(index, event, window, cx))
            .on_mouse_down(MouseButton::Right, move |event, window, cx| {
                cx.stop_propagation();
                on_right(index, event.position, window, cx)
            })
            .child(thumb)
            .child(name);
        let tile = self.drag.apply(tile);
        if self.marked {
            tile.bg(rgba((ACCENT << 8) | SELECTION_ALPHA))
        } else {
            tile.hover(|style| style.bg(rgb(ROW_HOVER)))
        }
    }
}
