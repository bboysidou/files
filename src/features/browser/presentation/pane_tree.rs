use std::path::Path;

use gpui::{Context, Div, InteractiveElement};

use crate::core::enums::view::view_mode::ViewMode;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::usecases::arrange_entries::arrange_entries;
use crate::features::browser::presentation::actions::{CollapseFolder, ExpandFolder};
use crate::features::browser::presentation::pane_view::PaneView;

const NO_FILTER: &str = "";

impl PaneView {
    pub(super) fn with_tree_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(cx.listener(|this, _: &ExpandFolder, _, cx| this.expand_folder(cx)))
            .on_action(cx.listener(|this, _: &CollapseFolder, _, cx| this.collapse_folder(cx)))
    }

    fn expand_folder(&mut self, cx: &mut Context<Self>) {
        let Some(entry) = self.current_entry().filter(|entry| entry.is_dir) else {
            return;
        };
        if self.view != ViewMode::List || self.search.is_some() {
            return;
        }
        if self.tree.children.contains_key(&entry.path) {
            self.tree.expanded.insert(entry.path);
            self.sync_tree_watches(cx);
            return self.rearrange(cx);
        }
        let repository = self.setup.repository.clone();
        let folder = entry.path.clone();
        self.tree.task = Some(cx.spawn(async move |this, cx| {
            let listed = cx
                .background_executor()
                .spawn(async move { repository.list(&folder).map(|children| (folder, children)) })
                .await;
            let _ = this.update(cx, |this, cx| {
                if let Ok((folder, children)) = listed {
                    this.pending_focus = Some(folder.clone());
                    this.tree.children.insert(folder.clone(), children);
                    this.tree.expanded.insert(folder);
                    this.sync_tree_watches(cx);
                    this.rearrange(cx);
                }
            });
        }));
    }

    fn collapse_folder(&mut self, cx: &mut Context<Self>) {
        let Some(entry) = self.current_entry() else {
            return;
        };
        if self.tree.expanded.remove(&entry.path) {
            self.tree
                .expanded
                .retain(|path| !path.starts_with(&entry.path));
            self.pending_focus = Some(entry.path);
            self.sync_tree_watches(cx);
            return self.rearrange(cx);
        }
        let Some(parent) = entry
            .path
            .parent()
            .filter(|parent| self.tree.expanded.contains(*parent))
        else {
            return;
        };
        let parent = parent.to_path_buf();
        self.tree.expanded.retain(|path| !path.starts_with(&parent));
        self.pending_focus = Some(parent);
        self.sync_tree_watches(cx);
        self.rearrange(cx);
    }

    pub(super) fn flatten_tree(&mut self, top: Vec<FileEntry>) -> Vec<FileEntry> {
        let mut flat = Vec::with_capacity(top.len());
        let mut depths = Vec::with_capacity(top.len());
        for entry in top {
            self.push_branch(entry, 0, (&mut flat, &mut depths));
        }
        self.tree.depths = depths;
        flat
    }

    fn push_branch(
        &self,
        entry: FileEntry,
        depth: usize,
        (flat, depths): (&mut Vec<FileEntry>, &mut Vec<usize>),
    ) {
        let open = self.tree.expanded.contains(&entry.path);
        let path = entry.path.clone();
        flat.push(entry);
        depths.push(depth);
        if !open {
            return;
        }
        for child in self.arranged_children(&path) {
            self.push_branch(child, depth + 1, (&mut *flat, &mut *depths));
        }
    }

    fn arranged_children(&self, folder: &Path) -> Vec<FileEntry> {
        self.tree
            .children
            .get(folder)
            .map(|children| arrange_entries(children, self.sort, self.show_hidden, NO_FILTER))
            .unwrap_or_default()
    }
}
