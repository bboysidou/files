use std::ops::Range;
use std::rc::Rc;
use std::time::SystemTime;

use gpui::{
    AnyElement, Context, IntoElement, ParentElement, Styled, UniformList, div, uniform_list,
};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::browser::sort_key::SortKey;
use crate::core::enums::view::button_tone::ButtonTone;
use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::metrics::{LIST_CELL_PAD_X, LIST_NAME_MIN_WIDTH};
use crate::features::browser::domain::entities::list_error::ListError;
use crate::features::browser::presentation::components::file_row::FileRow;
use crate::features::browser::presentation::components::list_header::ListHeader;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::handlers::SortHandler;
use crate::features::browser::presentation::state::load_state::LoadState;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::browser::presentation::state::row_click::RowClick;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::dialog_button::DialogButton;
use crate::global_components::empty_state::EmptyState;
use crate::global_components::handlers::{ClickHandler, IndexClickHandler};

impl PaneView {
    fn empty_state(icon: Icon, prefix: &str) -> AnyElement {
        let (title, body) = (format!("{prefix}.title"), format!("{prefix}.body"));
        EmptyState {
            icon,
            title: t!(&title).to_string().into(),
            body: t!(&body).to_string().into(),
            action: None,
        }
        .into_any_element()
    }

    fn denied_state(cx: &mut Context<Self>) -> AnyElement {
        let view = cx.entity().downgrade();
        let on_click: ClickHandler = Rc::new(move |_, cx| {
            let _ = view.update(cx, |_, cx| cx.emit(PaneEvent::AuthRequested));
        });
        EmptyState {
            icon: Icon::Folder,
            title: t!("empty.denied.title").to_string().into(),
            body: t!("empty.denied.body").to_string().into(),
            action: Some(
                DialogButton {
                    id: "authenticate",
                    label: t!("auth.authenticate").to_string().into(),
                    tone: ButtonTone::Accent,
                    on_click,
                }
                .into_any_element(),
            ),
        }
        .into_any_element()
    }

    fn no_matches(&self) -> AnyElement {
        EmptyState {
            icon: Icon::Filter,
            title: t!("empty.filter.title", query = self.filter)
                .to_string()
                .into(),
            body: t!("empty.filter.body").to_string().into(),
            action: None,
        }
        .into_any_element()
    }

    pub(super) fn body(&self, cx: &mut Context<Self>) -> AnyElement {
        match &self.load {
            LoadState::Unlisted if self.location == PlaceTarget::Overview => self.overview_page(cx),
            LoadState::Unlisted => Self::empty_state(Icon::Overview, "empty.overview"),
            LoadState::Failed(ListError::PermissionDenied) => Self::denied_state(cx),
            LoadState::Failed(ListError::NotFound) => {
                Self::empty_state(Icon::Folder, "empty.missing")
            }
            LoadState::Failed(ListError::Failed) => Self::empty_state(Icon::Folder, "empty.failed"),
            LoadState::Loading if self.entries.is_empty() => div().into_any_element(),
            LoadState::Ready(_) if self.entries.is_empty() && self.search.is_some() => {
                self.search_empty()
            }
            LoadState::Ready(_) if self.entries.is_empty() && !self.filter.is_empty() => {
                self.no_matches()
            }
            LoadState::Ready(_) if self.entries.is_empty() => {
                Self::empty_state(Icon::Folder, "empty.folder")
            }
            LoadState::Loading | LoadState::Ready(_) if self.view == ViewMode::Grid => {
                self.grid(cx)
            }
            LoadState::Loading | LoadState::Ready(_) if self.view == ViewMode::Columns => {
                self.columns(cx)
            }
            LoadState::Loading | LoadState::Ready(_) => self.file_list(cx),
        }
    }

    pub(super) fn entry_list(&self, cx: &mut Context<Self>) -> UniformList {
        uniform_list(
            "entries",
            self.entries.len(),
            cx.processor(|this, range: Range<usize>, _window, cx| this.rows(range, cx)),
        )
        .flex_1()
        .track_scroll(&self.scroll)
    }

    fn file_list(&self, cx: &mut Context<Self>) -> AnyElement {
        let view = cx.entity().downgrade();
        let on_sort: SortHandler = Rc::new(move |key: SortKey, _, cx| {
            let _ = view.update(cx, |this, cx| this.sort_by(key, cx));
        });
        let rows = self.entry_list(cx);
        div()
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .child(Self::width_probe(cx))
            .child(ListHeader {
                sort: self.sort,
                on_sort,
                columns: self.shown_columns(),
                on_resize: Self::resize_handler(cx),
            })
            .child(self.band_area(rows, cx))
            .into_any_element()
    }

    fn rows(&mut self, range: Range<usize>, cx: &mut Context<Self>) -> Vec<FileRow> {
        let view = cx.entity().downgrade();
        let on_click: IndexClickHandler = Rc::new(move |index, event, window, cx| {
            let _ = view.update(cx, |this, cx| {
                this.click_row(RowClick::from_event(index, event), window, cx)
            });
        });
        let on_right = Self::right_click_handler(cx);
        let now = SystemTime::now();
        let kit = self.drag_kit(cx);
        range
            .filter_map(|index| {
                let entry = self.entries.get(index)?;
                Some(FileRow {
                    index,
                    depth: self.tree.depth(index),
                    drag: self.drag_slot(&kit, index, entry),
                    entry: entry.clone(),
                    marked: self.selection.is_marked(index),
                    cursor: index == self.selection.cursor(),
                    focused: self.focused,
                    cut: self.cut.contains(&self.entries.get(index)?.path),
                    rename: (self.renames.path.as_ref() == Some(&self.entries.get(index)?.path))
                        .then(|| self.rename_field.clone()),
                    now,
                    on_click: on_click.clone(),
                    on_right: on_right.clone(),
                    columns: self.shown_columns(),
                    name_only: self.view == ViewMode::Columns,
                    context: self.search_context(&self.entries.get(index)?.path),
                })
            })
            .collect()
    }

    fn shown_columns(&self) -> Vec<(SortKey, u16)> {
        let available = self.body_width;
        let reserved = LIST_NAME_MIN_WIDTH + LIST_CELL_PAD_X * 2.0;
        self.list_columns.fitting(self.compact, available, reserved)
    }

    fn search_empty(&self) -> AnyElement {
        let prefix = match &self.search {
            Some(search) if search.index.is_none() => "search.indexing",
            Some(search) if search.query.trim().is_empty() => "search.prompt",
            _ => "search.none",
        };
        Self::empty_state(Icon::Search, prefix)
    }
}
