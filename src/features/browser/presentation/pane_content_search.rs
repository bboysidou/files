use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use gpui::Context;
use rust_i18n::t;

use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::search::domain::entities::content_match::ContentMatch;
use crate::features::search::domain::entities::indexed_path::IndexedPath;
use crate::features::search::domain::usecases::content_needle::content_needle;

impl PaneView {
    pub(super) fn cancel_content_search(&mut self) {
        if let Some(search) = self.search.as_mut() {
            search.cancel.store(true, Ordering::Relaxed);
            search.cancel = Arc::new(AtomicBool::new(false));
        }
    }

    pub(super) fn rerun_content_search(
        &mut self,
        index: Arc<Vec<IndexedPath>>,
        query: String,
        cx: &mut Context<Self>,
    ) {
        let Some(needle) = content_needle(&query).map(str::to_owned) else {
            return;
        };
        self.cancel_content_search();
        let Some(cancel) = self.search.as_ref().map(|search| search.cancel.clone()) else {
            return;
        };
        let (searcher, repository) = (self.setup.content.clone(), self.setup.repository.clone());
        self.search_task = Some(cx.spawn(async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn(async move {
                    searcher
                        .search(&index, &needle, &cancel)
                        .into_iter()
                        .filter_map(|hit| {
                            let entry = repository.entry(&hit.path)?;
                            Some((entry, content_snippet(&hit)))
                        })
                        .collect::<Vec<_>>()
                })
                .await;
            let _ = this.update(cx, |this, cx| {
                let Some(search) = this.search.as_mut().filter(|search| search.query == query)
                else {
                    return;
                };
                search.snippets = found
                    .iter()
                    .map(|(entry, snippet)| (entry.path.clone(), snippet.clone()))
                    .collect::<HashMap<_, _>>();
                search.results = found.into_iter().map(|(entry, _)| entry).collect();
                this.pending_focus = None;
                this.selection = Default::default();
                this.rearrange(cx);
            });
        }));
    }
}

fn content_snippet(hit: &ContentMatch) -> String {
    let snippet = format!("{}: {}", hit.line_number, hit.line);
    if hit.matches <= 1 {
        return snippet;
    }
    let more = t!("search.more_matches", count = hit.matches - 1);
    format!("{snippet}  {more}")
}
