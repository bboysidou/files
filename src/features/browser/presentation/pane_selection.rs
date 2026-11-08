use std::path::PathBuf;

use gpui::{Context, ScrollStrategy, Window};

use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::row_click::RowClick;

const DOUBLE_CLICK: usize = 2;

impl PaneView {
    pub(super) fn move_cursor(&mut self, delta: isize, cx: &mut Context<Self>) {
        let Some(last) = self.entries.len().checked_sub(1) else {
            return;
        };
        let next = self
            .selection
            .cursor()
            .saturating_add_signed(delta)
            .min(last);
        self.selection.move_to(next);
        self.reveal(cx);
    }

    pub(super) fn select_edge(&mut self, last: bool, cx: &mut Context<Self>) {
        let Some(final_index) = self.entries.len().checked_sub(1) else {
            return;
        };
        self.selection.move_to(if last { final_index } else { 0 });
        self.reveal(cx);
    }

    pub(super) fn click_row(
        &mut self,
        click: RowClick,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        let (index, modifiers) = (click.index, click.modifiers);
        if click.count >= DOUBLE_CLICK && !modifiers.shift && !modifiers.control {
            self.selection.move_to(index);
            self.open_entry(index, cx);
            return;
        }
        if modifiers.shift {
            self.selection.extend_to(index);
        } else if modifiers.control {
            self.selection.toggle(index);
        } else {
            self.selection.move_to(index);
        }
        cx.notify();
    }

    pub(super) fn reveal(&mut self, cx: &mut Context<Self>) {
        self.scroll
            .scroll_to_item(self.selection.cursor(), ScrollStrategy::Nearest);
        cx.notify();
    }

    pub(super) fn current_entry_path(&self) -> Option<PathBuf> {
        self.entries
            .get(self.selection.cursor())
            .map(|entry| entry.path.clone())
    }
}
