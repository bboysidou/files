use std::rc::Rc;

use gpui::{
    App, Context, Entity, FocusHandle, Focusable, IntoElement, Render, Subscription, Task,
    UniformListScrollHandle, Window, div, prelude::*,
};

use rust_i18n::t;

use crate::core::enums::view::view_mode::ViewMode;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::components::sheet::Sheet;
use crate::features::browser::presentation::key_bindings::BROWSER_CONTEXT;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::browser_setup::BrowserSetup;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;
use crate::features::browser::presentation::state::handlers::ViewHandler;
use crate::features::browser::presentation::state::menu_state::MenuState;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::browser::presentation::state::pane_start::PaneStart;
use crate::features::browser::presentation::state::preview_state::PreviewState;
use crate::features::browser::presentation::state::tab::Tab;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::browser::presentation::state::toast_model::ToastModel;
use crate::features::operations::domain::entities::clipboard::Clipboard;
use crate::features::operations::domain::entities::operation_history::OperationHistory;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};

pub struct BrowserView {
    pub(super) setup: Rc<BrowserSetup>,
    pub(super) tabs: Vec<Tab>,
    pub(super) active: usize,
    pub(super) options: ViewOptions,
    pub(super) clipboard: Option<Clipboard>,
    pub(super) history: OperationHistory,
    pub(super) toast: Option<ToastModel>,
    pub(super) toast_generation: usize,
    pub(super) toast_task: Option<Task<()>>,
    pub(super) operation_task: Option<Task<()>>,
    pub(super) paste_task: Option<Task<()>>,
    pub(super) quick_look: Option<PreviewState>,
    pub(super) quick_look_focus: FocusHandle,
    pub(super) quick_look_task: Option<Task<()>>,
    pub(super) quick_look_generation: usize,
    pub(super) preview_scroll: UniformListScrollHandle,
    pub(super) column_open: bool,
    pub(super) column_preview: Option<PreviewState>,
    pub(super) column_task: Option<Task<()>>,
    pub(super) column_generation: usize,
    pub(super) column_scroll: UniformListScrollHandle,
    pub(super) menu: Option<MenuState>,
    pub(super) menu_focus: FocusHandle,
    pub(super) keymap_open: bool,
    pub(super) keymap_focus: FocusHandle,
    pub(super) keymap_field: Entity<TextField>,
    pub(super) _keymap_events: Subscription,
}

impl BrowserView {
    pub fn new(
        setup: BrowserSetup,
        restore: BrowserSnapshot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (keymap_field, keymap_events) = Self::create_keymap_field(window, cx);
        let mut view = Self {
            setup: Rc::new(setup),
            tabs: Vec::new(),
            active: 0,
            options: restore.options,
            clipboard: None,
            history: OperationHistory::default(),
            toast: None,
            toast_generation: 0,
            toast_task: None,
            operation_task: None,
            paste_task: None,
            quick_look: None,
            quick_look_focus: cx.focus_handle(),
            quick_look_task: None,
            quick_look_generation: 0,
            preview_scroll: UniformListScrollHandle::new(),
            column_open: restore.preview_open,
            column_preview: None,
            column_task: None,
            column_generation: 0,
            column_scroll: UniformListScrollHandle::new(),
            menu: None,
            menu_focus: cx.focus_handle(),
            keymap_open: false,
            keymap_focus: cx.focus_handle(),
            keymap_field,
            _keymap_events: keymap_events,
        };
        let targets = if restore.tabs.is_empty() {
            vec![TabSnapshot::fresh(view.setup.home_target.clone())]
        } else {
            restore.tabs
        };
        for snapshot in targets {
            let tab = view.open_pane(snapshot, window, cx);
            view.tabs.push(tab);
        }
        view.active = restore.active.min(view.tabs.len() - 1);
        view.sync_preview_column(cx);
        view.sync_compact(cx);
        view
    }

    pub fn snapshot(&self, cx: &App) -> BrowserSnapshot {
        BrowserSnapshot {
            tabs: self
                .tabs
                .iter()
                .map(|tab| tab.pane.read(cx).tab_snapshot())
                .collect(),
            active: self.active,
            options: self.options,
            preview_open: self.column_open,
        }
    }

    pub fn start_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.active_pane()
            .clone()
            .update(cx, |pane, cx| pane.start_search(window, cx));
    }

    pub(super) fn edit_path(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.active_pane()
            .clone()
            .update(cx, |pane, cx| pane.start_path_entry(window, cx));
    }

    pub(super) fn set_view(&mut self, view: ViewMode, cx: &mut Context<Self>) {
        if view.is_available() && view != self.options.view {
            self.apply_view_options(
                ViewOptions {
                    view,
                    ..self.options
                },
                cx,
            );
        }
    }

    fn apply_view_options(&mut self, options: ViewOptions, cx: &mut Context<Self>) {
        self.options = options;
        for tab in &self.tabs {
            tab.pane
                .update(cx, |pane, cx| pane.set_view_options(options, cx));
        }
        cx.notify();
    }

    fn create_keymap_field(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<TextField>, Subscription) {
        let field = cx.new(|cx| TextField::new(t!("keys.placeholder").to_string(), cx));
        let events = cx.subscribe_in(&field, window, |this, _, event, window, cx| match event {
            TextFieldEvent::Changed => cx.notify(),
            TextFieldEvent::Submitted | TextFieldEvent::Cancelled => this.close_keymap(window, cx),
        });
        (field, events)
    }

    pub fn location(&self, cx: &App) -> PlaceTarget {
        self.active_pane().read(cx).location().clone()
    }

    pub fn navigate(&mut self, target: PlaceTarget, cx: &mut Context<Self>) {
        self.active_pane()
            .clone()
            .update(cx, |pane, cx| pane.navigate(target, cx));
    }

    pub(super) fn open_pane(
        &self,
        tab: TabSnapshot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Tab {
        let setup = self.setup.clone();
        let start = PaneStart {
            tab,
            options: self.options,
        };
        let pane = cx.new(|cx| PaneView::new(setup, start, window, cx));
        let changes = cx.observe(&pane, |this, _, cx| {
            this.sync_preview_column(cx);
            cx.notify();
        });
        let events =
            cx.subscribe_in(
                &pane,
                window,
                |this, _, event: &PaneEvent, window, cx| match event {
                    PaneEvent::RenameRequested(transfer) => this.rename(transfer.clone(), cx),
                    PaneEvent::ViewOptionsChanged(options) => this.apply_view_options(*options, cx),
                    PaneEvent::Message(message) => this.show_toast(message.clone(), false, cx),
                    PaneEvent::ContextMenu(position) => this.open_menu(*position, window, cx),
                },
            );
        Tab {
            pane,
            _changes: changes,
            _events: events,
        }
    }

    pub(super) fn active_pane(&self) -> &Entity<PaneView> {
        &self.tabs[self.active].pane
    }
}

impl Render for BrowserView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let quick_look = self.quick_look_overlay(window, cx);
        let menu = self.menu_overlay(window, cx);
        let keymap = self.keymap_overlay(window, cx);
        let pane = self.active_pane().clone();
        let toolbar = pane.update(cx, |pane, cx| pane.toolbar_model(cx));
        let (item_count, selected_count) = {
            let pane = pane.read(cx);
            (pane.item_count(), pane.selected_count())
        };
        let toast = self.toast_element(cx);
        let root = div()
            .relative()
            .key_context(BROWSER_CONTEXT)
            .flex()
            .flex_1()
            .min_w_0()
            .h_full();
        let side = self.preview_column(cx);
        let view = cx.entity().downgrade();
        let on_preview: ClickHandler = Rc::new(move |_, cx| {
            let _ = view.update(cx, |this, cx| this.toggle_preview_column(cx));
        });
        let keys_handle = cx.entity().downgrade();
        let on_keys: ClickHandler = Rc::new(move |window, cx| {
            let _ = keys_handle.update(cx, |this, cx| this.open_keymap(window, cx));
        });
        let view_handle = cx.entity().downgrade();
        let on_view: ViewHandler = Rc::new(move |view, _, cx| {
            let _ = view_handle.update(cx, |this, cx| this.set_view(view, cx));
        });
        Self::with_actions(root, cx)
            .child(Sheet {
                tabs: self.tab_strip_model(cx),
                toolbar,
                body: pane.into_any_element(),
                side,
                preview_open: self.column_open,
                on_preview,
                view: self.options.view,
                on_view,
                on_keys,
                item_count,
                selected_count,
            })
            .children(toast)
            .children(quick_look)
            .children(menu)
            .children(keymap)
    }
}

impl Focusable for BrowserView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane().focus_handle(cx)
    }
}
