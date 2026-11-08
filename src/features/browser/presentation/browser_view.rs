use std::rc::Rc;

use gpui::{
    AnyElement, App, Context, Div, Entity, FocusHandle, Focusable, IntoElement, Render,
    SharedString, Task, Window, div, prelude::*, px,
};

use crate::features::browser::presentation::actions::{
    ActivateTab1, ActivateTab2, ActivateTab3, ActivateTab4, ActivateTab5, ActivateTab6,
    ActivateTab7, ActivateTab8, ActivateTab9, CloseTab, CopySelection, CutSelection, NewFolder,
    NewTab, NextTab, PasteClipboard, PreviousTab, Redo, TrashSelection, Undo,
};
use rust_i18n::t;

use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::theme::metrics::TOAST_OFFSET;
use crate::features::browser::presentation::components::sheet::Sheet;
use crate::features::browser::presentation::key_bindings::BROWSER_CONTEXT;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::browser_setup::BrowserSetup;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::browser::presentation::state::tab::Tab;
use crate::features::browser::presentation::state::tab_strip_model::TabStripModel;
use crate::features::browser::presentation::state::toast_model::ToastModel;
use crate::features::operations::domain::entities::clipboard::Clipboard;
use crate::features::operations::domain::entities::operation_history::OperationHistory;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::toast::Toast;

pub struct BrowserView {
    pub(super) setup: Rc<BrowserSetup>,
    pub(super) tabs: Vec<Tab>,
    pub(super) active: usize,
    pub(super) home_target: PlaceTarget,
    pub(super) clipboard: Option<Clipboard>,
    pub(super) history: OperationHistory,
    pub(super) toast: Option<ToastModel>,
    pub(super) toast_generation: usize,
    pub(super) toast_task: Option<Task<()>>,
    pub(super) operation_task: Option<Task<()>>,
}

impl BrowserView {
    pub fn new(
        setup: BrowserSetup,
        home_target: PlaceTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut view = Self {
            setup: Rc::new(setup),
            tabs: Vec::new(),
            active: 0,
            home_target,
            clipboard: None,
            history: OperationHistory::default(),
            toast: None,
            toast_generation: 0,
            toast_task: None,
            operation_task: None,
        };
        let first = view.open_pane(view.home_target.clone(), window, cx);
        view.tabs.push(first);
        view
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
        let pane = cx.new(|cx| PaneView::new(setup, target, window, cx));
        let changes = cx.observe(&pane, |_, _, cx| cx.notify());
        let events = cx.subscribe(&pane, |this, _, event: &PaneEvent, cx| match event {
            PaneEvent::RenameRequested(transfer) => this.rename(transfer.clone(), cx),
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

    fn tab_strip_model(&self, cx: &mut Context<Self>) -> TabStripModel {
        let (select_view, close_view, new_view) = (
            cx.entity().downgrade(),
            cx.entity().downgrade(),
            cx.entity().downgrade(),
        );
        let on_select: IndexHandler = Rc::new(move |index, window, cx| {
            let _ = select_view.update(cx, |this, cx| this.activate(index, window, cx));
        });
        let on_close: IndexHandler = Rc::new(move |index, window, cx| {
            let _ = close_view.update(cx, |this, cx| this.close_tab(index, window, cx));
        });
        let on_new: ClickHandler = Rc::new(move |window, cx| {
            let _ = new_view.update(cx, |this, cx| this.new_tab(window, cx));
        });
        TabStripModel {
            titles: self
                .tabs
                .iter()
                .map(|tab| tab.pane.read(cx).title())
                .collect(),
            active: self.active,
            on_select,
            on_close,
            on_new,
        }
    }

    fn toast_element(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let toast = self.toast.clone()?;
        let view = cx.entity().downgrade();
        let undo: ClickHandler = Rc::new(move |_, cx| {
            let _ = view.update(cx, |this, cx| this.undo(cx));
        });
        let action = toast
            .can_undo
            .then(|| (SharedString::from(t!("op.undo").to_string()), undo));
        Some(
            div()
                .absolute()
                .bottom(px(TOAST_OFFSET))
                .left_0()
                .right_0()
                .flex()
                .justify_center()
                .child(Toast {
                    message: toast.message,
                    action,
                })
                .into_any_element(),
        )
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
        Self::with_actions(root, cx)
            .child(Sheet {
                tabs: self.tab_strip_model(cx),
                toolbar,
                body: pane.into_any_element(),
                item_count,
                selected_count,
            })
            .children(toast)
    }
}

impl Focusable for BrowserView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane().focus_handle(cx)
    }
}
