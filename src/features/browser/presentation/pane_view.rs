use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;

use gpui::{
    App, Context, Div, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Render,
    SharedString, Subscription, Task, UniformListScrollHandle, Window, div, prelude::*,
};
use rust_i18n::t;

use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::domain::entities::navigation_history::NavigationHistory;
use crate::features::browser::domain::entities::selection::Selection;
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::presentation::actions::{
    Dismiss, ExtendDown, ExtendUp, GoBack, GoForward, GoParent, MoveDown, MoveUp, OpenEntry,
    OpenFilter, Reload, RenameEntry, SelectAllEntries, SelectFirst, SelectLast, ToggleHidden,
    ToggleMark,
};
use crate::features::browser::presentation::key_bindings::PANE_CONTEXT;
use crate::features::browser::presentation::state::browser_setup::BrowserSetup;
use crate::features::browser::presentation::state::load_state::LoadState;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::browser::presentation::state::pane_start::PaneStart;
use crate::features::browser::presentation::state::toolbar_model::ToolbarModel;
use crate::features::places::domain::entities::crumb_trail::CrumbTrail;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::text_field::text_field_view::TextField;

pub struct PaneView {
    pub(super) setup: Rc<BrowserSetup>,
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
    pub(super) filter_field: Entity<TextField>,
    pub(super) filter: String,
    pub(super) filter_open: bool,
    pub(super) _filter_events: Subscription,
    pub(super) cut: HashSet<PathBuf>,
    pub(super) renaming: Option<PathBuf>,
    pub(super) rename_field: Entity<TextField>,
    pub(super) focus_rename: bool,
    pub(super) pending_rename: Option<PathBuf>,
    pub(super) _rename_events: Subscription,
    pub(super) watched: Option<PathBuf>,
    pub(super) watch_task: Option<Task<()>>,
    pub(super) compact: bool,
}

impl PaneView {
    pub fn new(
        setup: Rc<BrowserSetup>,
        start: PaneStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let filter_field = cx.new(|cx| TextField::new(t!("filter.placeholder").to_string(), cx));
        let filter_events = cx.subscribe_in(&filter_field, window, |this, _, event, window, cx| {
            this.on_filter_event(*event, window, cx)
        });
        let rename_field = cx.new(|cx| TextField::new("", cx));
        let rename_events = cx.subscribe_in(&rename_field, window, |this, _, event, window, cx| {
            this.on_rename_event(*event, window, cx)
        });
        let mut view = Self {
            setup,
            location: start.target,
            history: NavigationHistory::default(),
            load: LoadState::Loading,
            entries: Vec::new(),
            selection: Selection::default(),
            sort: start.options.sort,
            show_hidden: start.options.show_hidden,
            pending_focus: None,
            generation: 0,
            scroll: UniformListScrollHandle::new(),
            focus_handle: cx.focus_handle(),
            load_task: None,
            filter_field,
            filter: String::new(),
            filter_open: false,
            _filter_events: filter_events,
            cut: HashSet::new(),
            renaming: None,
            rename_field,
            focus_rename: false,
            pending_rename: None,
            _rename_events: rename_events,
            watched: None,
            watch_task: None,
            compact: false,
        };
        view.load(cx);
        view
    }

    pub fn location(&self) -> &PlaceTarget {
        &self.location
    }

    pub(super) fn item_count(&self) -> usize {
        self.entries.len()
    }

    pub(super) fn selected_count(&self) -> usize {
        self.selection.marked_count()
    }

    pub(super) fn title(&self) -> SharedString {
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

    pub(super) fn toolbar_model(&self, cx: &mut Context<Self>) -> ToolbarModel {
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
            filter: self.filter_model(),
            on_back: Self::handler(cx, Self::go_back),
            on_forward: Self::handler(cx, Self::go_forward),
            on_up: Self::handler(cx, Self::go_parent),
            on_filter: Self::open_filter_handler(cx),
            on_crumb,
        }
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
            .on_action(cx.listener(|this, _: &OpenFilter, window, cx| this.open_filter(window, cx)))
            .on_action(cx.listener(|this, _: &Dismiss, window, cx| this.dismiss(window, cx)))
            .on_action(cx.listener(|this, _: &ToggleMark, _, cx| this.toggle_mark(cx)))
            .on_action(cx.listener(|this, _: &SelectAllEntries, _, cx| this.select_all(cx)))
            .on_action(cx.listener(|this, _: &ExtendDown, _, cx| this.extend(1, cx)))
            .on_action(cx.listener(|this, _: &ExtendUp, _, cx| this.extend(-1, cx)))
            .on_action(cx.listener(|this, _: &RenameEntry, _, cx| this.start_rename(cx)))
    }
}

impl Render for PaneView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_rename {
            self.focus_rename = false;
            window.focus(&self.rename_field.focus_handle(cx), cx);
        } else if !self.filter_open && self.filter_field.focus_handle(cx).is_focused(window) {
            window.focus(&self.focus_handle, cx);
        }
        let root = div()
            .track_focus(&self.focus_handle)
            .key_context(PANE_CONTEXT)
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .size_full();
        Self::with_actions(root, cx).child(self.body(cx))
    }
}

impl EventEmitter<PaneEvent> for PaneView {}

impl Focusable for PaneView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
