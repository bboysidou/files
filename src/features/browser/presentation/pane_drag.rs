use std::path::PathBuf;
use std::rc::Rc;

use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, WeakEntity, div};

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::drop_request::DropRequest;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::global_components::drag::accept_paths::accept_paths;
use crate::global_components::drag::drag_slot::DragSlot;
use crate::global_components::drag::dragged_files::DraggedFiles;
use crate::global_components::handlers::PathsHandler;

pub struct DragKit {
    view: WeakEntity<PaneView>,
    marked: DraggedFiles,
}

impl PaneView {
    pub(super) fn drag_kit(&self, cx: &Context<Self>) -> DragKit {
        let entries = self
            .entries
            .iter()
            .enumerate()
            .filter(|(index, _)| self.selection.is_marked(*index))
            .map(|(_, entry)| (entry.path.clone(), entry.is_dir))
            .collect();
        DragKit {
            view: cx.entity().downgrade(),
            marked: DraggedFiles {
                entries: Rc::new(entries),
            },
        }
    }

    pub(super) fn drag_slot(&self, kit: &DragKit, index: usize, entry: &FileEntry) -> DragSlot {
        let files = if self.selection.is_marked(index) {
            kit.marked.clone()
        } else {
            DraggedFiles {
                entries: Rc::new(vec![(entry.path.clone(), entry.is_dir)]),
            }
        };
        let on_drop = entry
            .is_dir
            .then(|| drop_into(kit.view.clone(), entry.path.clone()));
        DragSlot { files, on_drop }
    }

    pub(super) fn drop_zone(&self, body: AnyElement, cx: &Context<Self>) -> AnyElement {
        let Some(directory) = self.directory() else {
            return body;
        };
        let zone = div().flex().flex_col().flex_1().min_h_0().size_full();
        accept_paths(zone, drop_into(cx.entity().downgrade(), directory))
            .child(body)
            .into_any_element()
    }
}

fn drop_into(view: WeakEntity<PaneView>, destination: PathBuf) -> PathsHandler {
    Rc::new(move |sources, window, cx| {
        let request = DropRequest {
            sources,
            destination: destination.clone(),
            copy: window.modifiers().control,
        };
        let _ = view.update(cx, |_, cx| cx.emit(PaneEvent::DropRequested(request)));
    })
}
