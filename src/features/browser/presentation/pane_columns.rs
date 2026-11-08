use std::path::{Path, PathBuf};
use std::rc::Rc;

use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, div};

use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::metrics::MILLER_COLUMN_LIMIT;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::usecases::arrange_entries::arrange_entries;
use crate::features::browser::presentation::components::miller_column::MillerColumn;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::IndexHandler;

const NO_FILTER: &str = "";

type ColumnPick = fn(&mut PaneView, usize, &mut Context<PaneView>);

impl PaneView {
    pub(super) fn columns(&self, cx: &mut Context<Self>) -> AnyElement {
        let child = (!self.columns.child.is_empty()).then(|| MillerColumn {
            id: "miller-child",
            entries: self.columns.child.clone(),
            highlight: None,
            on_select: Self::column_handler(cx, Self::open_child_column),
            on_drop: self.column_drop_handler(&self.columns.child, cx),
        });
        div()
            .size_full()
            .flex()
            .child(MillerColumn {
                id: "miller-parent",
                entries: self.columns.parent.clone(),
                highlight: self.directory(),
                on_select: Self::column_handler(cx, Self::open_parent_column),
                on_drop: self.column_drop_handler(&self.columns.parent, cx),
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .child(self.band_area(self.entry_list(cx), cx)),
            )
            .children(child)
            .into_any_element()
    }

    fn column_handler(cx: &mut Context<Self>, pick: ColumnPick) -> IndexHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |index, _, cx| {
            let _ = view.update(cx, |this, cx| pick(this, index, cx));
        })
    }

    fn open_column_entry(&mut self, entry: Option<FileEntry>, cx: &mut Context<Self>) {
        let Some(entry) = entry else {
            return;
        };
        let (folder, focus) = if entry.is_dir {
            (entry.path.clone(), None)
        } else {
            let Some(parent) = entry.path.parent().map(Path::to_path_buf) else {
                return;
            };
            (parent, Some(entry.path.clone()))
        };
        let leaving = self.current_stop();
        self.history.visit(leaving);
        self.go_to(PlaceTarget::Directory(folder), focus, cx);
    }

    fn open_parent_column(&mut self, index: usize, cx: &mut Context<Self>) {
        let entry = self.columns.parent.get(index).cloned();
        self.open_column_entry(entry, cx);
    }

    fn open_child_column(&mut self, index: usize, cx: &mut Context<Self>) {
        let entry = self.columns.child.get(index).cloned();
        self.open_column_entry(entry, cx);
    }

    fn column_targets(&self) -> (Option<PathBuf>, Option<PathBuf>) {
        let parent = self
            .directory()
            .and_then(|folder| folder.parent().map(Path::to_path_buf));
        let child = self
            .entries
            .get(self.selection.cursor())
            .filter(|entry| entry.is_dir)
            .map(|entry| entry.path.clone());
        (parent, child)
    }

    pub(super) fn sync_columns(&mut self, cx: &mut Context<Self>) {
        if self.view != ViewMode::Columns {
            return;
        }
        let (parent_of, child_of) = self.column_targets();
        if parent_of == self.columns.parent_of && child_of == self.columns.child_of {
            return;
        }
        self.columns.parent_of = parent_of.clone();
        self.columns.child_of = child_of.clone();
        let (repository, sort, hidden) =
            (self.setup.repository.clone(), self.sort, self.show_hidden);
        let list = move |folder: Option<PathBuf>| -> Vec<FileEntry> {
            let entries = folder
                .and_then(|folder| repository.list(&folder).ok())
                .unwrap_or_default();
            let mut arranged = arrange_entries(&entries, sort, hidden, NO_FILTER);
            arranged.truncate(MILLER_COLUMN_LIMIT);
            arranged
        };
        self.columns_task = Some(cx.spawn(async move |this, cx| {
            let (parent, child) = cx
                .background_executor()
                .spawn(async move { (list(parent_of), list(child_of)) })
                .await;
            let _ = this.update(cx, |this, cx| {
                this.columns.parent = parent;
                this.columns.child = child;
                cx.notify();
            });
        }));
    }
}
