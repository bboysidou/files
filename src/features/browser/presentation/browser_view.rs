use std::rc::Rc;

use gpui::{
    App, Context, Div, Entity, FocusHandle, Focusable, IntoElement, Render, Task,
    UniformListScrollHandle, Window, div, prelude::*,
};

use crate::features::browser::presentation::actions::{
    ActivateTab1, ActivateTab2, ActivateTab3, ActivateTab4, ActivateTab5, ActivateTab6,
    ActivateTab7, ActivateTab8, ActivateTab9, CloseTab, CopySelection, CutSelection, NewFolder,
    NewTab, NextTab, PasteClipboard, PreviousTab, QuickLook, Redo, TogglePreview, TrashSelection,
    Undo,
};

use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::components::sheet::Sheet;
use crate::features::browser::presentation::key_bindings::BROWSER_CONTEXT;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::browser_setup::BrowserSetup;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::browser::presentation::state::pane_start::PaneStart;
use crate::features::browser::presentation::state::preview_state::PreviewState;
use crate::features::browser::presentation::state::tab::Tab;
use crate::features::browser::presentation::state::toast_model::ToastModel;
use crate::features::operations::domain::entities::clipboard::Clipboard;
use crate::features::operations::domain::entities::operation_history::OperationHistory;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::ClickHandler;

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
}

impl BrowserView {
    pub fn new(
        setup: BrowserSetup,
        restore: BrowserSnapshot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
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
        };
        let targets = if restore.tabs.is_empty() {
            vec![view.setup.home_target.clone()]
        } else {
            restore.tabs
        };
        for target in targets {
            let tab = view.open_pane(target, window, cx);
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
                .map(|tab| tab.pane.read(cx).location().clone())
                .collect(),
            active: self.active,
            options: self.options,
            preview_open: self.column_open,
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
        target: PlaceTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Tab {
        let setup = self.setup.clone();
        let start = PaneStart {
            target,
            options: self.options,
        };
        let pane = cx.new(|cx| PaneView::new(setup, start, window, cx));
        let changes = cx.observe(&pane, |this, _, cx| {
            this.sync_preview_column(cx);
            cx.notify();
        });
        let events = cx.subscribe(&pane, |this, _, event: &PaneEvent, cx| match event {
            PaneEvent::RenameRequested(transfer) => this.rename(transfer.clone(), cx),
            PaneEvent::ViewOptionsChanged(options) => this.apply_view_options(*options, cx),
        });
        Tab {
            pane,
            _changes: changes,
            _events: events,
        }
    }

    pub(super) fn active_pane(&self) -> &Entity<PaneView> {
        &self.tabs[self.active].pane
    }

    fn with_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(cx.listener(|this, _: &CopySelection, _, cx| {
                this.copy_to_clipboard(ClipboardMode::Copy, cx)
            }))
            .on_action(cx.listener(|this, _: &CutSelection, _, cx| {
                this.copy_to_clipboard(ClipboardMode::Cut, cx)
            }))
            .on_action(cx.listener(|this, _: &PasteClipboard, _, cx| this.paste(cx)))
            .on_action(cx.listener(|this, _: &TrashSelection, _, cx| this.trash_selection(cx)))
            .on_action(cx.listener(|this, _: &NewFolder, _, cx| this.new_folder(cx)))
            .on_action(cx.listener(|this, _: &Undo, _, cx| this.undo(cx)))
            .on_action(cx.listener(|this, _: &TogglePreview, _, cx| this.toggle_preview_column(cx)))
            .on_action(
                cx.listener(|this, _: &QuickLook, window, cx| this.open_quick_look(window, cx)),
            )
            .on_action(cx.listener(|this, _: &Redo, _, cx| this.redo(cx)))
            .on_action(cx.listener(|this, _: &NewTab, window, cx| this.new_tab(window, cx)))
            .on_action(cx.listener(|this, _: &CloseTab, window, cx| this.close_active(window, cx)))
            .on_action(cx.listener(|this, _: &NextTab, window, cx| this.cycle(1, window, cx)))
            .on_action(cx.listener(|this, _: &PreviousTab, window, cx| this.cycle(-1, window, cx)))
            .on_action(
                cx.listener(|this, _: &ActivateTab1, window, cx| this.activate(0, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab2, window, cx| this.activate(1, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab3, window, cx| this.activate(2, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab4, window, cx| this.activate(3, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab5, window, cx| this.activate(4, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab6, window, cx| this.activate(5, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab7, window, cx| this.activate(6, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab8, window, cx| this.activate(7, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &ActivateTab9, window, cx| this.activate(8, window, cx)),
            )
    }
}

impl Render for BrowserView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let quick_look = self.quick_look_overlay(window, cx);
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
        Self::with_actions(root, cx)
            .child(Sheet {
                tabs: self.tab_strip_model(cx),
                toolbar,
                body: pane.into_any_element(),
                side,
                preview_open: self.column_open,
                on_preview,
                item_count,
                selected_count,
            })
            .children(toast)
            .children(quick_look)
    }
}

impl Focusable for BrowserView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane().focus_handle(cx)
    }
}
