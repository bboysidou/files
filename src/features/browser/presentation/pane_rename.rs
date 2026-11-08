use std::path::PathBuf;

use gpui::{Context, Window};

use crate::core::constants::files::{FORBIDDEN_NAME_CHARS, RESERVED_NAMES};
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::operations::domain::entities::file_operation::Transfer;
use crate::global_components::text_field::text_field_view::TextFieldEvent;

impl PaneView {
    pub(super) fn start_rename(&mut self, cx: &mut Context<Self>) {
        if self.selection.marked_count() > 1 {
            return cx.emit(PaneEvent::BulkRenameRequested(self.target_paths()));
        }
        if let Some(path) = self.current_entry_path() {
            self.begin_rename_of(path, cx);
        }
    }

    pub(super) fn begin_rename_of(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let Some(entry) = self.entries.iter().find(|entry| entry.path == path) else {
            return;
        };
        let name = entry.name.clone();
        let stem_end = if entry.is_dir {
            name.len()
        } else {
            name.rfind('.').filter(|dot| *dot > 0).unwrap_or(name.len())
        };
        self.rename_field
            .update(cx, |field, cx| field.set_text(&name, 0..stem_end, cx));
        self.renaming = Some(path);
        self.focus_rename = true;
        cx.notify();
    }

    pub(super) fn on_rename_event(
        &mut self,
        event: TextFieldEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            TextFieldEvent::Changed => {}
            TextFieldEvent::Submitted => self.commit_rename(window, cx),
            TextFieldEvent::Cancelled => self.stop_rename(window, cx),
        }
    }

    fn commit_rename(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(from) = self.renaming.clone() else {
            return;
        };
        let name = self.rename_field.read(cx).text().trim().to_string();
        self.stop_rename(window, cx);
        let invalid = name.is_empty()
            || name.contains(FORBIDDEN_NAME_CHARS)
            || RESERVED_NAMES.contains(&name.as_str());
        let Some(parent) = from.parent().filter(|_| !invalid) else {
            return;
        };
        let to = parent.join(&name);
        if to != from {
            cx.emit(PaneEvent::RenameRequested(Transfer { from, to }));
        }
    }

    fn stop_rename(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.renaming = None;
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }
}
