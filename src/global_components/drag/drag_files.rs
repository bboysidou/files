use gpui::{
    AppContext, ExternalDragPayload, FileDragPaths, SharedString, StatefulInteractiveElement,
};
use rust_i18n::t;

use crate::global_components::drag::drag_preview::DragPreview;
use crate::global_components::drag::dragged_files::DraggedFiles;

pub fn drag_files<E: StatefulInteractiveElement>(element: E, files: DraggedFiles) -> E {
    element
        .on_drag(files, |files, _, _, cx| {
            let label = drag_label(files);
            cx.new(|_| DragPreview { label })
        })
        .external_drag_payload(|files: &DraggedFiles, _, _| {
            let entries = files.entries.iter().cloned();
            Some(ExternalDragPayload::Files(FileDragPaths::new(entries)))
        })
}

fn drag_label(files: &DraggedFiles) -> SharedString {
    match files.entries.as_slice() {
        [(path, _)] => path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default()
            .into(),
        entries => t!("drag.items", count = entries.len()).to_string().into(),
    }
}
