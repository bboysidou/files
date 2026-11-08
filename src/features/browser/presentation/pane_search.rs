use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use gpui::{Context, Focusable, Window};
use rust_i18n::t;

use crate::core::constants::search::{CONTEXT_SEPARATOR, SEARCH_DEBOUNCE_MS, SEARCH_RESULT_LIMIT};
use crate::core::dependency_injection::tags::tag_store;
use crate::core::enums::tags::tag_color::TagColor;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::filter_model::FilterModel;
use crate::features::browser::presentation::state::search_state::SearchState;
use crate::features::search::domain::usecases::content_needle::content_needle;
use crate::features::search::domain::usecases::find_matches::find_matches;
use crate::features::tags::domain::usecases::parse_tag_query::tag_query;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::text_field::text_field_view::TextFieldEvent;

impl PaneView {
    pub fn start_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search.is_none() {
            let Some(root) = self.directory().or_else(|| self.setup.home.clone()) else {
                return;
            };
            self.begin_search(root, cx);
        }
        window.focus(&self.search_field.focus_handle(cx), cx);
        cx.notify();
    }

    pub fn search_tag(&mut self, tag: TagColor, window: &mut Window, cx: &mut Context<Self>) {
        let Some(home) = self.setup.home.clone() else {
            return;
        };
        self.end_search(window, cx);
        self.begin_search(home, cx);
        let query = tag_query(tag);
        self.search_field.update(cx, |field, cx| {
            field.set_text(&query, query.len()..query.len(), cx)
        });
        if let Some(search) = self.search.as_mut() {
            search.query = query;
        }
        window.focus(&self.search_field.focus_handle(cx), cx);
        cx.notify();
    }

    fn begin_search(&mut self, root: PathBuf, cx: &mut Context<Self>) {
        self.reset_filter(cx);
        self.search_field.update(cx, |field, cx| field.clear(cx));
        self.search = Some(SearchState {
            root: root.clone(),
            query: String::new(),
            index: None,
            results: Vec::new(),
            return_focus: self.current_entry_path(),
            snippets: Default::default(),
            cancel: Default::default(),
        });
        self.build_index(root, cx);
        self.rearrange(cx);
    }

    fn build_index(&mut self, root: PathBuf, cx: &mut Context<Self>) {
        let (indexer, include_hidden) = (self.setup.indexer.clone(), self.show_hidden);
        self.index_task = Some(cx.spawn(async move |this, cx| {
            let index = cx
                .background_executor()
                .spawn(async move { indexer.index(&root, include_hidden) })
                .await;
            let _ = this.update(cx, |this, cx| {
                if let Some(search) = this.search.as_mut() {
                    search.index = Some(Arc::new(index));
                    this.rerun_search(cx);
                }
            });
        }));
    }

    pub(super) fn on_search_event(
        &mut self,
        event: TextFieldEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            TextFieldEvent::Changed => {
                let query = self.search_field.read(cx).text().to_string();
                if let Some(search) = self.search.as_mut() {
                    search.query = query;
                }
                self.schedule_search(cx);
            }
            TextFieldEvent::Submitted => {
                window.focus(&self.focus_handle, cx);
                cx.notify();
            }
            TextFieldEvent::Cancelled => self.end_search(window, cx),
        }
    }

    fn schedule_search(&mut self, cx: &mut Context<Self>) {
        self.search_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(SEARCH_DEBOUNCE_MS))
                .await;
            let _ = this.update(cx, |this, cx| this.rerun_search(cx));
        }));
    }

    fn rerun_search(&mut self, cx: &mut Context<Self>) {
        let Some((Some(index), query)) = self
            .search
            .as_ref()
            .map(|search| (search.index.clone(), search.query.clone()))
        else {
            return;
        };
        if content_needle(&query).is_some() {
            return self.rerun_content_search(index, query, cx);
        }
        self.cancel_content_search();
        if let Some(search) = self.search.as_mut() {
            search.snippets.clear();
        }
        let repository = self.setup.repository.clone();
        let wanted = query.clone();
        let store = tag_store();
        self.search_task = Some(cx.spawn(async move |this, cx| {
            let results = cx
                .background_executor()
                .spawn(async move {
                    let tags_of = |path: &std::path::Path| TagColor::from_names(&store.names(path));
                    find_matches(&index, &wanted, SEARCH_RESULT_LIMIT, &tags_of)
                        .iter()
                        .filter_map(|path| repository.entry(path))
                        .collect::<Vec<_>>()
                })
                .await;
            let _ = this.update(cx, |this, cx| {
                let Some(search) = this.search.as_mut().filter(|search| search.query == query)
                else {
                    return;
                };
                search.results = results;
                this.pending_focus = None;
                this.selection = Default::default();
                this.rearrange(cx);
            });
        }));
    }

    pub(super) fn end_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_content_search();
        let Some(search) = self.search.take() else {
            return;
        };
        self.search_task = None;
        self.index_task = None;
        self.search_field.update(cx, |field, cx| field.clear(cx));
        self.pending_focus = search.return_focus;
        self.selection = Default::default();
        self.rearrange(cx);
        window.focus(&self.focus_handle, cx);
    }

    pub(super) fn search_model(&self) -> Option<FilterModel> {
        self.search.as_ref().map(|search| FilterModel {
            field: self.search_field.clone(),
            count: t!("search.results", count = search.results.len())
                .to_string()
                .into(),
        })
    }

    pub(super) fn search_context(&self, path: &std::path::Path) -> Option<String> {
        let search = self.search.as_ref()?;
        let parent = path.parent()?.strip_prefix(&search.root).ok()?;
        let folder = parent.to_string_lossy().into_owned();
        match search.snippets.get(path) {
            Some(snippet) if folder.is_empty() => Some(snippet.clone()),
            Some(snippet) => Some(format!("{folder}{CONTEXT_SEPARATOR}{snippet}")),
            None => Some(folder).filter(|text| !text.is_empty()),
        }
    }

    pub(super) fn start_search_handler(cx: &mut Context<Self>) -> ClickHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.start_search(window, cx));
        })
    }
}
