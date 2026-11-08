use std::path::PathBuf;

use gpui::Context;

use crate::core::enums::browser::sort_key::SortKey;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::list_error::ListError;
use crate::features::browser::domain::entities::navigation_history::HistoryStop;
use crate::features::browser::domain::entities::selection::Selection;
use crate::features::browser::domain::usecases::arrange_entries::arrange_entries;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::load_state::LoadState;
use crate::features::places::domain::entities::place::PlaceTarget;

impl PaneView {
    pub fn navigate(&mut self, target: PlaceTarget, cx: &mut Context<Self>) {
        if target == self.location {
            return;
        }
        let leaving = self.current_stop();
        self.history.visit(leaving);
        self.go_to(target, None, cx);
    }

    pub(super) fn go_to(
        &mut self,
        target: PlaceTarget,
        focus: Option<PathBuf>,
        cx: &mut Context<Self>,
    ) {
        self.location = target;
        self.pending_focus = focus;
        self.reset_filter(cx);
        self.selection = Selection::default();
        self.entries.clear();
        self.load(cx);
    }

    pub(super) fn load(&mut self, cx: &mut Context<Self>) {
        self.generation += 1;
        let Some(directory) = self.setup.repository.resolve(&self.location) else {
            self.load = LoadState::Unlisted;
            self.load_task = None;
            self.entries.clear();
            cx.notify();
            return;
        };
        self.load = LoadState::Loading;
        let (repository, generation) = (self.setup.repository.clone(), self.generation);
        self.load_task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { repository.list(&directory) })
                .await;
            let _ = this.update(cx, |this, cx| this.finish_load(generation, result, cx));
        }));
        cx.notify();
    }

    fn finish_load(
        &mut self,
        generation: usize,
        result: Result<Vec<FileEntry>, ListError>,
        cx: &mut Context<Self>,
    ) {
        if generation != self.generation {
            return;
        }
        match result {
            Ok(raw) => {
                self.load = LoadState::Ready(raw);
                self.rearrange(cx);
            }
            Err(error) => {
                self.load = LoadState::Failed(error);
                self.entries.clear();
                cx.notify();
            }
        }
    }

    pub(super) fn rearrange(&mut self, cx: &mut Context<Self>) {
        let LoadState::Ready(raw) = &self.load else {
            return;
        };
        let focus = self
            .pending_focus
            .take()
            .or_else(|| self.current_entry_path());
        self.entries = arrange_entries(raw, self.sort, self.show_hidden, &self.filter);
        let cursor = focus
            .and_then(|focus| self.entries.iter().position(|entry| entry.path == focus))
            .unwrap_or_default();
        self.selection = Selection::at(cursor);
        self.reveal(cx);
    }

    pub(super) fn go_back(&mut self, cx: &mut Context<Self>) {
        let leaving = self.current_stop();
        if let Some(stop) = self.history.back(leaving) {
            self.go_to(stop.target, stop.focus, cx);
        }
    }

    pub(super) fn go_forward(&mut self, cx: &mut Context<Self>) {
        let leaving = self.current_stop();
        if let Some(stop) = self.history.forward(leaving) {
            self.go_to(stop.target, stop.focus, cx);
        }
    }

    pub(super) fn parent_target(&self) -> Option<PlaceTarget> {
        match &self.location {
            PlaceTarget::Directory(path) => path
                .parent()
                .map(|parent| PlaceTarget::Directory(parent.to_path_buf())),
            PlaceTarget::Overview | PlaceTarget::Trash => None,
        }
    }

    pub(super) fn go_parent(&mut self, cx: &mut Context<Self>) {
        let (PlaceTarget::Directory(child), Some(parent)) =
            (self.location.clone(), self.parent_target())
        else {
            return;
        };
        let leaving = self.current_stop();
        self.history.visit(leaving);
        self.go_to(parent, Some(child), cx);
    }

    pub(super) fn open_cursor(&mut self, cx: &mut Context<Self>) {
        self.open_entry(self.selection.cursor(), cx);
    }

    pub(super) fn open_entry(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(entry) = self.entries.get(index) else {
            return;
        };
        if entry.is_dir {
            self.navigate(PlaceTarget::Directory(entry.path.clone()), cx);
        } else {
            self.setup.launcher.launch(&entry.path);
        }
    }

    pub(super) fn toggle_hidden(&mut self, cx: &mut Context<Self>) {
        self.show_hidden = !self.show_hidden;
        self.rearrange(cx);
    }

    pub(super) fn sort_by(&mut self, key: SortKey, cx: &mut Context<Self>) {
        self.sort = self.sort.toggled_on(key);
        self.rearrange(cx);
    }

    fn current_stop(&self) -> HistoryStop {
        HistoryStop {
            target: self.location.clone(),
            focus: self.current_entry_path(),
        }
    }
}
