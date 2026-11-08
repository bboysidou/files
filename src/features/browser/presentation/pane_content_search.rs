use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use gpui::Context;
use rust_i18n::t;

use crate::core::constants::search::{CONTENT_LINE_LIMIT, CONTENT_SNIPPET_CHARS};
use crate::core::theme::colors::ACCENT;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::preview::domain::entities::preview_content::PreviewContent;
use crate::features::search::domain::entities::content_match::ContentMatch;
use crate::features::search::domain::entities::indexed_path::IndexedPath;
use crate::features::search::domain::usecases::content_needle::content_needle;
use crate::features::search::domain::usecases::match_preview_lines::match_preview_lines;

impl PaneView {
    pub fn content_preview(&self, path: &Path) -> Option<PreviewContent> {
        let search = self.search.as_ref()?;
        let needle = content_needle(&search.query)?.to_lowercase();
        let hit = search.hits.get(path)?;
        Some(PreviewContent::Text {
            lines: match_preview_lines(hit, &needle, ACCENT),
            truncated: hit.lines.len() >= CONTENT_LINE_LIMIT,
        })
    }

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
                            Some((entry, hit))
                        })
                        .collect::<Vec<_>>()
                })
                .await;
            let _ = this.update(cx, |this, cx| {
                let Some(search) = this.search.as_mut().filter(|search| search.query == query)
                else {
                    return;
                };
                search.hits = found
                    .iter()
                    .map(|(entry, hit)| (entry.path.clone(), hit.clone()))
                    .collect::<HashMap<_, _>>();
                search.results = found.into_iter().map(|(entry, _)| entry).collect();
                this.pending_focus = None;
                this.selection = Default::default();
                this.rearrange(cx);
            });
        }));
    }
}

pub(super) fn content_snippet(hit: &ContentMatch) -> String {
    let Some(first) = hit.lines.first() else {
        return String::new();
    };
    let text: String = first.text.chars().take(CONTENT_SNIPPET_CHARS).collect();
    let snippet = format!("{}: {text}", first.number);
    if hit.matches <= 1 {
        return snippet;
    }
    let more = t!("search.more_matches", count = hit.matches - 1);
    format!("{snippet}  {more}")
}
