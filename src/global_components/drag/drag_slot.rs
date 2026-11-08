use gpui::StatefulInteractiveElement;

use crate::global_components::drag::accept_paths::accept_paths;
use crate::global_components::drag::drag_files::drag_files;
use crate::global_components::drag::dragged_files::DraggedFiles;
use crate::global_components::handlers::PathsHandler;

pub struct DragSlot {
    pub files: DraggedFiles,
    pub on_drop: Option<PathsHandler>,
}

impl DragSlot {
    pub fn apply<E: StatefulInteractiveElement>(self, element: E) -> E {
        let element = drag_files(element, self.files);
        match self.on_drop {
            Some(on_drop) => accept_paths(element, on_drop),
            None => element,
        }
    }
}
