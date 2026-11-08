use std::collections::HashSet;
use std::path::PathBuf;

use std::rc::Rc;

use gpui::{Context, Pixels, Point, Window};

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::IndexPointHandler;

impl PaneView {
    pub(crate) fn target_paths(&self) -> Vec<PathBuf> {
        let marked: Vec<PathBuf> = self
            .selection
            .marked()
            .filter_map(|index| self.entries.get(index))
            .map(|entry| entry.path.clone())
            .collect();
        if marked.is_empty() {
            self.current_entry_path().into_iter().collect()
        } else {
            marked
        }
    }

    pub(super) fn current_entry(&self) -> Option<FileEntry> {
        self.entries.get(self.selection.cursor()).cloned()
    }

    pub(crate) fn directory(&self) -> Option<PathBuf> {
        match &self.location {
            PlaceTarget::Directory(path) => Some(path.clone()),
            PlaceTarget::Overview | PlaceTarget::Trash => None,
        }
    }

    pub(super) fn set_cut(&mut self, cut: HashSet<PathBuf>, cx: &mut Context<Self>) {
        self.cut = cut;
        cx.notify();
    }

    pub(super) fn reload_with_focus(
        &mut self,
        focus: Option<PathBuf>,
        rename: bool,
        cx: &mut Context<Self>,
    ) {
        if focus.is_some() {
            self.pending_focus = focus.clone();
        }
        self.pending_rename = focus.filter(|_| rename);
        self.load(cx);
    }

    pub(super) fn toggle_mark(&mut self, cx: &mut Context<Self>) {
        if !self.entries.is_empty() {
            self.selection.toggle(self.selection.cursor());
            cx.notify();
        }
    }

    pub(super) fn select_all(&mut self, cx: &mut Context<Self>) {
        self.selection.mark_all(self.entries.len());
        cx.notify();
    }

    pub(super) fn extend(&mut self, delta: isize, cx: &mut Context<Self>) {
        let Some(last) = self.entries.len().checked_sub(1) else {
            return;
        };
        let next = self
            .selection
            .cursor()
            .saturating_add_signed(delta)
            .min(last);
        self.selection.extend_to(next);
        self.reveal(cx);
    }

    pub(super) fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search.is_some() {
            self.end_search(window, cx);
        } else if self.filter_open || !self.filter.is_empty() {
            self.clear_filter(window, cx);
        } else if self.picker && self.selection.marked().next().is_none() {
            cx.emit(PaneEvent::PickCancelled);
        } else {
            let cursor = self.selection.cursor();
            self.selection.move_to(cursor);
            cx.propagate();
            cx.notify();
        }
    }

    pub(super) fn right_click(
        &mut self,
        (index, position): (usize, Point<Pixels>),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.selection.is_marked(index) {
            self.selection.move_to(index);
        }
        self.request_menu(position, window, cx);
    }

    pub(super) fn request_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        cx.emit(PaneEvent::ContextMenu(position));
        cx.notify();
    }

    pub(super) fn right_click_handler(cx: &mut Context<Self>) -> IndexPointHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |index, position, window, cx| {
            let _ = view.update(cx, |this, cx| {
                this.right_click((index, position), window, cx)
            });
        })
    }

    pub(super) fn focus_when_loaded(&mut self, path: PathBuf) {
        self.pending_focus = Some(path);
    }

    pub(crate) fn set_picker(&mut self, picker: bool) {
        self.picker = picker;
    }
}
