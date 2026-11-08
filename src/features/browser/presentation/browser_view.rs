use std::rc::Rc;

use gpui::{
    App, Context, Entity, FocusHandle, Focusable, IntoElement, Render, Subscription, Task, Window,
    div, prelude::*,
};

use crate::core::enums::tags::tag_color::TagColor;
use crate::core::enums::view::view_mode::ViewMode;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::components::sheet::Sheet;
use crate::features::browser::presentation::key_bindings::BROWSER_CONTEXT;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::browser_event::BrowserEvent;
use crate::features::browser::presentation::state::browser_parts::{
    ColumnSlot, Jobs, QuickLookSlot, Toasts,
};
use crate::features::browser::presentation::state::browser_setup::BrowserSetup;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;
use crate::features::browser::presentation::state::bulk_rename_state::BulkRenameState;
use crate::features::browser::presentation::state::confirm_state::ConfirmState;
use crate::features::browser::presentation::state::dual_pane::DualPane;
use crate::features::browser::presentation::state::menu_state::MenuState;
use crate::features::browser::presentation::state::network_login_state::{
    NetworkLoginRequest, NetworkLoginState,
};
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::browser::presentation::state::pane_start::PaneStart;
use crate::features::browser::presentation::state::properties_state::PropertiesState;
use crate::features::browser::presentation::state::settings_sheet::SettingsSheet;
use crate::features::browser::presentation::state::tab::Tab;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::operations::domain::entities::clipboard::Clipboard;
use crate::features::operations::domain::entities::operation_history::OperationHistory;
use crate::global_components::text_field::text_field_view::TextField;

pub struct BrowserView {
    pub(super) setup: Rc<BrowserSetup>,
    pub(super) tabs: Vec<Tab>,
    pub(super) active: usize,
    pub(super) options: ViewOptions,
    pub(super) clipboard: Option<Clipboard>,
    pub(super) history: OperationHistory,
    pub(super) toasts: Toasts,
    pub(super) jobs: Jobs,
    pub(super) quick_look: QuickLookSlot,
    pub(super) column: ColumnSlot,
    pub(super) menu: Option<MenuState>,
    pub(super) menu_focus: FocusHandle,
    pub(super) open_with_task: Option<Task<()>>,
    pub(super) settings: SettingsSheet,
    pub(super) bulk: Option<BulkRenameState>,
    pub(super) network_login: Option<NetworkLoginState>,
    pub(super) properties: Option<PropertiesState>,
    pub(super) keymap_open: bool,
    pub(super) keymap_focus: FocusHandle,
    pub(super) keymap_field: Entity<TextField>,
    pub(super) _keymap_events: Subscription,
    pub(super) confirm: Option<ConfirmState>,
    pub(super) confirm_focus: FocusHandle,
    pub(super) trash_task: Option<Task<()>>,
    pub(super) focus_confirm: bool,
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
            dual: DualPane::restored(restore.dual_split),
            active: 0,
            options: restore.options,
            clipboard: None,
            history: OperationHistory::default(),
            toasts: Toasts::default(),
            jobs: Jobs::default(),
            quick_look: QuickLookSlot::new(cx.focus_handle()),
            column: ColumnSlot::new(restore.preview_open),
            menu: None,
            menu_focus: cx.focus_handle(),
            open_with_task: None,
            bulk: None,
            network_login: None,
            properties: None,
            settings: SettingsSheet::closed(cx.focus_handle(), &restore),
            keymap_open: false,
            keymap_focus: cx.focus_handle(),
            keymap_field,
            _keymap_events: keymap_events,
            confirm: None,
            confirm_focus: cx.focus_handle(),
            trash_task: None,
            focus_confirm: false,
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
        self.remember_view(view, cx);
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

    pub(super) fn apply_view_options(&mut self, options: ViewOptions, cx: &mut Context<Self>) {
        self.options = options;
        for tab in &self.tabs {
            tab.pane
                .update(cx, |pane, cx| pane.set_view_options(options, cx));
        }
        cx.notify();
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
        let events = cx.subscribe_in(
            &pane,
            window,
            |this, pane, event: &PaneEvent, window, cx| match event {
                PaneEvent::RenameRequested(transfer) => this.rename(transfer.clone(), cx),
                PaneEvent::ViewOptionsChanged(options) => this.apply_view_options(*options, cx),
                PaneEvent::Message(message) => this.show_toast(message.clone(), false, cx),
                PaneEvent::ContextMenu(position) => this.open_menu(*position, window, cx),
                PaneEvent::RestoreRequested(entries) => {
                    this.restore_from_trash(entries.clone(), cx)
                }
                PaneEvent::DropRequested(request) => this.drop_files(request.clone(), cx),
                PaneEvent::Navigated(folder) => this.recall_folder_view(folder, cx),
                PaneEvent::CredentialsRequested { uri, rejected } => this.open_network_login(
                    NetworkLoginRequest {
                        pane: pane.downgrade(),
                        uri: uri.clone(),
                        rejected: *rejected,
                    },
                    window,
                    cx,
                ),
                PaneEvent::BulkRenameRequested(paths) => {
                    this.open_bulk_rename(paths.clone(), window, cx)
                }
                PaneEvent::Connected { uri, replaces } => cx.emit(BrowserEvent::ServerConnected {
                    uri: uri.clone(),
                    replaces: replaces.clone(),
                }),
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
        let sheets = [
            self.menu_overlay(window, cx),
            self.keymap_overlay(window, cx),
            self.settings_overlay(window, cx),
            self.bulk_overlay(window, cx),
            self.network_login_overlay(window, cx),
            self.properties_overlay(window, cx),
        ];
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
        let (on_preview, on_keys, on_view, on_settings) = Self::chrome_handlers(cx);
        Self::with_dual_actions(Self::with_actions(root, cx), cx)
            .child(Sheet {
                tabs: self.tab_strip_model(cx),
                toolbar,
                body,
                side,
                preview_open: self.column.open,
                on_preview,
                view: self.options.view,
                on_view,
                on_keys,
                on_settings,
                item_count,
                selected_count,
            })
            .children(toast)
            .children(quick_look)
            .children(sheets.into_iter().flatten())
            .children(confirm)
    }
}

impl Focusable for BrowserView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane().focus_handle(cx)
    }
}

impl gpui::EventEmitter<BrowserEvent> for BrowserView {}
