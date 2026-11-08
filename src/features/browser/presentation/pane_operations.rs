use std::collections::HashSet;
use std::path::PathBuf;

use gpui::{Context, Window};

use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::places::domain::entities::place::PlaceTarget;

impl PaneView {
    pub(super) fn target_paths(&self) -> Vec<PathBuf> {
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

    pub(super) fn directory(&self) -> Option<PathBuf> {
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
        if self.filter_open || !self.filter.is_empty() {
            self.clear_filter(window, cx);
        } else {
            let cursor = self.selection.cursor();
            self.selection.move_to(cursor);
            cx.notify();
        }
    }
}
