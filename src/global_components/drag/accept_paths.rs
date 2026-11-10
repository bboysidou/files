use gpui::{ExternalPaths, InteractiveElement, Styled, rgba};

use crate::core::theme::colors::{ACCENT, DROP_ALPHA};
use crate::global_components::drag::dragged_files::DraggedFiles;
use crate::global_components::drag::marker_drop::drop_marker;
use crate::global_components::handlers::PathsHandler;

pub fn accept_paths<E: InteractiveElement>(element: E, on_drop: PathsHandler) -> E {
    let external = on_drop.clone();
    let tint = rgba((ACCENT << 8) | DROP_ALPHA);
    element
        .drag_over::<DraggedFiles>(move |style, _, _, _| style.bg(tint))
        .drag_over::<ExternalPaths>(move |style, _, _, _| style.bg(tint))
        .on_drop(move |files: &DraggedFiles, window, cx| on_drop(files.paths(), window, cx))
        .on_drop(move |paths: &ExternalPaths, window, cx| {
            if !drop_marker(paths.paths(), window, cx) {
                external(paths.paths().to_vec(), window, cx)
            }
        })
}
