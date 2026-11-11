use std::path::PathBuf;

use gpui::Context;

use crate::core::constants::thumbnails::DIRECT_IMAGE_EXTENSIONS;
use crate::core::enums::files::file_kind::FileKind;
use crate::core::helpers::files::is_image::is_image;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::thumb_state::{
    ThumbEntry, ThumbRequest, ThumbState,
};
use crate::features::thumbnails::domain::entities::thumbnail::Thumbnail;

impl PaneView {
    pub(super) fn thumbnail_for(
        &mut self,
        entry: &FileEntry,
        cx: &mut Context<Self>,
    ) -> Option<PathBuf> {
        let extension = entry.extension.as_deref();
        if !is_image(extension) && entry.kind != FileKind::Video {
            return None;
        }
        let direct = extension.is_some_and(|extension| {
            DIRECT_IMAGE_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str())
        });
        let Some(modified) = entry.modified.filter(|_| !direct) else {
            return Some(entry.path.clone());
        };
        match self.thumbnails.entries.get(&entry.path) {
            Some(known) if known.modified == modified => match &known.state {
                ThumbState::Ready(path) => Some(path.clone()),
                ThumbState::Pending | ThumbState::Failed => None,
            },
            _ => {
                let pending = ThumbEntry {
                    modified,
                    state: ThumbState::Pending,
                };
                self.thumbnails.entries.insert(entry.path.clone(), pending);
                self.thumbnails.queue.push_back(ThumbRequest {
                    path: entry.path.clone(),
                    modified,
                });
                self.ensure_thumb_worker(cx);
                None
            }
        }
    }

    pub(super) fn reset_thumbnails(&mut self) {
        self.thumbnails.entries.clear();
        self.thumbnails.queue.clear();
        self.thumbnails.worker = None;
    }

    fn ensure_thumb_worker(&mut self, cx: &mut Context<Self>) {
        if self.thumbnails.worker.is_some() {
            return;
        }
        let store = self.setup.thumbnails.clone();
        self.thumbnails.worker = Some(cx.spawn(async move |this, cx| {
            loop {
                let next = this
                    .update(cx, |this, _| {
                        let next = this.thumbnails.queue.pop_front();
                        if next.is_none() {
                            this.thumbnails.worker = None;
                        }
                        next
                    })
                    .ok()
                    .flatten();
                let Some(request) = next else {
                    break;
                };
                let (store, source, modified) =
                    (store.clone(), request.path.clone(), request.modified);
                let thumbnail = cx
                    .background_executor()
                    .spawn(async move { store.thumbnail(&source, modified) })
                    .await;
                let _ = this.update(cx, |this, cx| this.finish_thumbnail(request, thumbnail, cx));
            }
        }));
    }

    fn finish_thumbnail(
        &mut self,
        request: ThumbRequest,
        thumbnail: Thumbnail,
        cx: &mut Context<Self>,
    ) {
        let (path, modified) = (request.path, request.modified);
        let Some(known) = self
            .thumbnails
            .entries
            .get_mut(&path)
            .filter(|known| known.modified == modified)
        else {
            return;
        };
        known.state = match thumbnail {
            Thumbnail::Ready(file) => ThumbState::Ready(file),
            Thumbnail::Failed => ThumbState::Failed,
        };
        cx.notify();
    }
}
