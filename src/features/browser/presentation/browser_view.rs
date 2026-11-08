use std::ops::Range;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::SystemTime;

use gpui::{
    AnyElement, App, Context, Div, FocusHandle, Focusable, IntoElement, Render, SharedString, Task,
    UniformListScrollHandle, Window, div, prelude::*, uniform_list,
};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::enums::browser::sort_key::SortKey;
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::list_error::ListError;
use crate::features::browser::domain::entities::navigation_history::NavigationHistory;
use crate::features::browser::domain::entities::selection::Selection;
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::browser::domain::repositories::file_launcher::FileLauncher;
use crate::features::browser::presentation::actions::{
    GoBack, GoForward, GoParent, MoveDown, MoveUp, OpenEntry, Reload, SelectFirst, SelectLast,
    ToggleHidden,
};
use crate::features::browser::presentation::components::file_row::FileRow;
use crate::features::browser::presentation::components::list_header::ListHeader;
use crate::features::browser::presentation::components::sheet::Sheet;
use crate::features::browser::presentation::key_bindings::BROWSER_CONTEXT;
use crate::features::browser::presentation::state::handlers::SortHandler;
use crate::features::browser::presentation::state::load_state::LoadState;
use crate::features::browser::presentation::state::row_click::RowClick;
use crate::features::browser::presentation::state::toolbar_model::ToolbarModel;
use crate::features::places::domain::entities::crumb_trail::CrumbTrail;
use crate::features::places::domain::entities::place::{Place, PlaceTarget};
use crate::global_components::empty_state::EmptyState;
use crate::global_components::handlers::{ClickHandler, IndexClickHandler, IndexHandler};

pub struct BrowserSetup {
    pub repository: Arc<dyn DirectoryRepository>,
    pub launcher: Rc<dyn FileLauncher>,
    pub places: Vec<Place>,
    pub home: Option<PathBuf>,
}

pub struct BrowserView {
    pub(super) setup: BrowserSetup,
    pub(super) location: PlaceTarget,
    pub(super) history: NavigationHistory,
    pub(super) load: LoadState,
    pub(super) entries: Vec<FileEntry>,
    pub(super) selection: Selection,
    pub(super) sort: SortOrder,
    pub(super) show_hidden: bool,
    pub(super) pending_focus: Option<PathBuf>,
    pub(super) generation: usize,
    pub(super) scroll: UniformListScrollHandle,
    pub(super) focus_handle: FocusHandle,
    pub(super) load_task: Option<Task<()>>,
}

impl BrowserView {
    pub fn new(setup: BrowserSetup, start: PlaceTarget, cx: &mut Context<Self>) -> Self {
        let mut view = Self {
            setup,
            location: start,
            history: NavigationHistory::default(),
            load: LoadState::Loading,
            entries: Vec::new(),
            selection: Selection::default(),
            sort: SortOrder::by_name(),
            show_hidden: false,
            pending_focus: None,
            generation: 0,
            scroll: UniformListScrollHandle::new(),
            focus_handle: cx.focus_handle(),
            load_task: None,
        };
        view.load(cx);
        view
    }

    pub fn location(&self) -> &PlaceTarget {
        &self.location
    }

    fn title(&self) -> SharedString {
        if let Some(place) = self
            .setup
            .places
            .iter()
            .find(|place| place.target == self.location)
        {
            return t!(place.kind.label_key()).to_string().into();
        }
        match &self.location {
            PlaceTarget::Directory(path) => path.file_name().map_or_else(
                || self.trail().display_path(),
                |name| name.to_string_lossy().into_owned(),
            ),
            _ => self.trail().display_path(),
        }
        .into()
    }

    fn trail(&self) -> CrumbTrail {
        CrumbTrail::for_target(&self.location, self.setup.home.as_deref())
    }

    fn handler(cx: &mut Context<Self>, action: fn(&mut Self, &mut Context<Self>)) -> ClickHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |_, cx| {
            let _ = view.update(cx, action);
        })
    }

    fn toolbar_model(&self, cx: &mut Context<Self>) -> ToolbarModel {
        let trail = self.trail();
        let view = cx.entity().downgrade();
        let targets = trail.targets.clone();
        let on_crumb: IndexHandler = Rc::new(move |index, _, cx| {
            let Some(target) = targets.get(index).cloned() else {
                return;
            };
            let _ = view.update(cx, |this, cx| this.navigate(target, cx));
        });
        ToolbarModel {
            trail,
            can_go_back: self.history.can_go_back(),
            can_go_forward: self.history.can_go_forward(),
            can_go_up: self.parent_target().is_some(),
            on_back: Self::handler(cx, Self::go_back),
            on_forward: Self::handler(cx, Self::go_forward),
            on_up: Self::handler(cx, Self::go_parent),
            on_crumb,
        }
    }

    fn empty_state(icon: Icon, prefix: &str) -> AnyElement {
        let (title, body) = (format!("{prefix}.title"), format!("{prefix}.body"));
        EmptyState {
            icon,
            title: t!(&title).to_string().into(),
            body: t!(&body).to_string().into(),
        }
        .into_any_element()
    }

    fn body(&self, cx: &mut Context<Self>) -> AnyElement {
        match &self.load {
            LoadState::Unlisted => Self::empty_state(Icon::Overview, "empty.overview"),
            LoadState::Failed(ListError::PermissionDenied) => {
                Self::empty_state(Icon::Folder, "empty.denied")
            }
            LoadState::Failed(ListError::NotFound) => {
                Self::empty_state(Icon::Folder, "empty.missing")
            }
            LoadState::Failed(ListError::Failed) => Self::empty_state(Icon::Folder, "empty.failed"),
            LoadState::Loading if self.entries.is_empty() => div().into_any_element(),
            LoadState::Ready(_) if self.entries.is_empty() => {
                Self::empty_state(Icon::Folder, "empty.folder")
            }
            LoadState::Loading | LoadState::Ready(_) => self.file_list(cx),
        }
    }

    fn file_list(&self, cx: &mut Context<Self>) -> AnyElement {
        let view = cx.entity().downgrade();
        let on_sort: SortHandler = Rc::new(move |key: SortKey, _, cx| {
            let _ = view.update(cx, |this, cx| this.sort_by(key, cx));
        });
        let rows = uniform_list(
            "entries",
            self.entries.len(),
            cx.processor(|this, range: Range<usize>, _window, cx| this.rows(range, cx)),
        )
        .flex_1()
        .track_scroll(&self.scroll);
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(ListHeader {
                sort: self.sort,
                on_sort,
            })
            .child(rows)
            .into_any_element()
    }

    fn rows(&mut self, range: Range<usize>, cx: &mut Context<Self>) -> Vec<FileRow> {
        let view = cx.entity().downgrade();
        let on_click: IndexClickHandler = Rc::new(move |index, event, window, cx| {
            let _ = view.update(cx, |this, cx| {
                this.click_row(RowClick::from_event(index, event), window, cx)
            });
        });
        let now = SystemTime::now();
        range
            .filter_map(|index| {
                Some(FileRow {
                    index,
                    entry: self.entries.get(index)?.clone(),
                    marked: self.selection.is_marked(index),
                    cursor: index == self.selection.cursor(),
                    now,
                    on_click: on_click.clone(),
                })
            })
            .collect()
    }

    fn with_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(cx.listener(|this, _: &MoveDown, _, cx| this.move_cursor(1, cx)))
            .on_action(cx.listener(|this, _: &MoveUp, _, cx| this.move_cursor(-1, cx)))
            .on_action(cx.listener(|this, _: &SelectFirst, _, cx| this.select_edge(false, cx)))
            .on_action(cx.listener(|this, _: &SelectLast, _, cx| this.select_edge(true, cx)))
            .on_action(cx.listener(|this, _: &OpenEntry, _, cx| this.open_cursor(cx)))
            .on_action(cx.listener(|this, _: &GoParent, _, cx| this.go_parent(cx)))
            .on_action(cx.listener(|this, _: &GoBack, _, cx| this.go_back(cx)))
            .on_action(cx.listener(|this, _: &GoForward, _, cx| this.go_forward(cx)))
            .on_action(cx.listener(|this, _: &ToggleHidden, _, cx| this.toggle_hidden(cx)))
            .on_action(cx.listener(|this, _: &Reload, _, cx| this.load(cx)))
    }
}

impl Render for BrowserView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let root = div()
            .track_focus(&self.focus_handle)
            .key_context(BROWSER_CONTEXT)
            .flex()
            .flex_1()
            .min_w_0()
            .h_full();
        Self::with_actions(root, cx).child(Sheet {
            title: self.title(),
            toolbar: self.toolbar_model(cx),
            body: self.body(cx),
            item_count: self.entries.len(),
            selected_count: self.selection.marked_count(),
        })
    }
}

impl Focusable for BrowserView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
