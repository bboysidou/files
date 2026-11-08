use std::collections::VecDeque;
use std::rc::Rc;

use gpui::{
    App, Context, Entity, FocusHandle, Focusable, IntoElement, Render, Subscription, Task,
    UniformListScrollHandle, Window, div, prelude::*,
};

use rust_i18n::t;

use crate::core::enums::tags::tag_color::TagColor;
use crate::core::enums::view::view_mode::ViewMode;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::components::sheet::Sheet;
use crate::features::browser::presentation::key_bindings::BROWSER_CONTEXT;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::browser_setup::BrowserSetup;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;
use crate::features::browser::presentation::state::confirm_state::ConfirmState;
use crate::features::browser::presentation::state::dual_pane::DualPane;
use crate::features::browser::presentation::state::job_state::JobState;
use crate::features::browser::presentation::state::menu_state::MenuState;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::browser::presentation::state::pane_start::PaneStart;
use crate::features::browser::presentation::state::preview_state::PreviewState;
use crate::features::browser::presentation::state::tab::Tab;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::browser::presentation::state::toast_model::ToastModel;
use crate::features::operations::domain::entities::clipboard::Clipboard;
use crate::features::operations::domain::entities::file_operation::FileOperation;
use crate::features::operations::domain::entities::operation_history::OperationHistory;
use crate::features::places::domain::entities::place::PlaceTarget;
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
    pub(super) open_with_task: Option<Task<()>>,
    pub(super) keymap_open: bool,
    pub(super) keymap_focus: FocusHandle,
    pub(super) keymap_field: Entity<TextField>,
    pub(super) _keymap_events: Subscription,
    pub(super) confirm: Option<ConfirmState>,
    pub(super) confirm_focus: FocusHandle,
    pub(super) trash_task: Option<Task<()>>,
    pub(super) focus_confirm: bool,
    pub(super) job: Option<JobState>,
    pub(super) job_ticker: Option<Task<()>>,
    pub(super) queue: VecDeque<(FileOperation, OperationIntent)>,
    pub(super) dual: DualPane,
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
            dual: DualPane::default(),
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
            open_with_task: None,
            keymap_open: false,
            keymap_focus: cx.focus_handle(),
            keymap_field,
            _keymap_events: keymap_events,
            confirm: None,
            confirm_focus: cx.focus_handle(),
            trash_task: None,
            focus_confirm: false,
            job: None,
            job_ticker: None,
            queue: VecDeque::new(),
        };
        view.restore_tabs((restore.tabs, restore.active), window, cx);
        view.sync_preview_column(cx);
        view.sync_compact(cx);
        view
    }

    pub fn start_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.active_pane()
            .clone()
            .update(cx, |pane, cx| pane.start_search(window, cx));
    }

    pub fn search_tag(&mut self, tag: TagColor, window: &mut Window, cx: &mut Context<Self>) {
        self.active_pane()
            .clone()
            .update(cx, |pane, cx| pane.search_tag(tag, window, cx));
    }

    pub(super) fn edit_path(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.active_pane()
            .clone()
            .update(cx, |pane, cx| pane.start_path_entry(window, cx));
    }

    pub(super) fn set_view(&mut self, view: ViewMode, cx: &mut Context<Self>) {
        if view != self.options.view {
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

    pub fn notify_message(&mut self, message: String, cx: &mut Context<Self>) {
        self.show_toast(message, false, cx);
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
                    PaneEvent::RestoreRequested(entries) => {
                        this.restore_from_trash(entries.clone(), cx)
                    }
                    PaneEvent::DropRequested(request) => this.drop_files(request.clone(), cx),
                    PaneEvent::Picked(_) | PaneEvent::PickCancelled => {}
                },
            );
        Tab {
            pane,
            _changes: changes,
            _events: events,
        }
    }

    pub(super) fn active_pane(&self) -> &Entity<PaneView> {
        match &self.dual.partner {
            Some(partner) if self.dual.focused && self.options.view == ViewMode::Dual => {
                &partner.pane
            }
            _ => &self.tabs[self.active].pane,
        }
    }
}

impl Render for BrowserView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_confirm {
            self.focus_confirm = false;
            window.focus(&self.confirm_focus, cx);
        }
        let body = self.pane_body(window, cx);
        let quick_look = self.quick_look_overlay(window, cx);
        let menu = self.menu_overlay(window, cx);
        let keymap = self.keymap_overlay(window, cx);
        let confirm = self.confirm_overlay(window, cx);
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
        let (on_preview, on_keys, on_view) = Self::chrome_handlers(cx);
        Self::with_dual_actions(Self::with_actions(root, cx), cx)
            .child(Sheet {
                tabs: self.tab_strip_model(cx),
                toolbar,
                body,
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
            .children(confirm)
    }
}

impl Focusable for BrowserView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane().focus_handle(cx)
    }
}
