use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use gpui::{Context, Div, InteractiveElement};

use crate::features::browser::presentation::actions::{InvertSelection, MeasureFolders};

use crate::core::helpers::files::device_tree_size::device_tree_size;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::pane_view::PaneView;

impl PaneView {
    pub(super) fn with_size_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(cx.listener(|this, _: &InvertSelection, _, cx| this.invert_selection(cx)))
            .on_action(cx.listener(|this, _: &MeasureFolders, _, cx| this.compute_folder_sizes(cx)))
    }

    pub(super) fn invert_selection(&mut self, cx: &mut Context<Self>) {
        self.selection.invert(self.entries.len());
        cx.notify();
    }

    pub(super) fn compute_folder_sizes(&mut self, cx: &mut Context<Self>) {
        let marked: Vec<PathBuf> = self
            .selection
            .marked()
            .filter_map(|index| self.entries.get(index))
            .filter(|entry| entry.is_dir)
            .map(|entry| entry.path.clone())
            .collect();
        let folders = if marked.is_empty() {
            self.entries
                .iter()
                .filter(|entry| entry.is_dir)
                .map(|entry| entry.path.clone())
                .collect()
        } else {
            marked
        };
        self.sizes.task = Some(cx.spawn(async move |this, cx| {
            let measured = cx
                .background_executor()
                .spawn(async move {
                    folders
                        .into_iter()
                        .filter_map(|folder| {
                            let device = fs::metadata(&folder).ok()?.dev();
                            let size = device_tree_size(&folder, device);
                            Some((folder, size))
                        })
                        .collect::<Vec<_>>()
                })
                .await;
            let _ = this.update(cx, |this, cx| {
                this.sizes.known.extend(measured);
                this.sizes.task = None;
                this.rearrange(cx);
                cx.notify();
            });
        }));
    }

    pub(super) fn with_folder_sizes(&self, raw: &[FileEntry]) -> Vec<FileEntry> {
        raw.iter()
            .map(|entry| match self.sizes.known.get(&entry.path) {
                Some(size) if entry.is_dir => FileEntry {
                    size: Some(*size),
                    ..entry.clone()
                },
                _ => entry.clone(),
            })
            .collect()
    }
}
