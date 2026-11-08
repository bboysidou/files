use std::rc::Rc;

use gpui::{
    App, Context, Div, Entity, FocusHandle, Focusable, IntoElement, Render, Window, div, prelude::*,
};

use crate::features::browser::presentation::actions::{
    ActivateTab1, ActivateTab2, ActivateTab3, ActivateTab4, ActivateTab5, ActivateTab6,
    ActivateTab7, ActivateTab8, ActivateTab9, CloseTab, NewTab, NextTab, PreviousTab,
};
use crate::features::browser::presentation::components::sheet::Sheet;
use crate::features::browser::presentation::key_bindings::BROWSER_CONTEXT;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::browser_setup::BrowserSetup;
use crate::features::browser::presentation::state::tab::Tab;
use crate::features::browser::presentation::state::tab_strip_model::TabStripModel;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::{ClickHandler, IndexHandler};

pub struct BrowserView {
    pub(super) setup: Rc<BrowserSetup>,
    pub(super) tabs: Vec<Tab>,
    pub(super) active: usize,
    pub(super) home_target: PlaceTarget,
}

impl BrowserView {
    pub fn new(setup: BrowserSetup, home_target: PlaceTarget, cx: &mut Context<Self>) -> Self {
        let mut view = Self {
            setup: Rc::new(setup),
            tabs: Vec::new(),
            active: 0,
            home_target,
        };
        let first = view.open_pane(view.home_target.clone(), cx);
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

    pub(super) fn open_pane(&self, target: PlaceTarget, cx: &mut Context<Self>) -> Tab {
        let setup = self.setup.clone();
        let pane = cx.new(|cx| PaneView::new(setup, target, cx));
        let changes = cx.observe(&pane, |_, _, cx| cx.notify());
        Tab {
            pane,
            _changes: changes,
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

    fn with_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
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
        let root = div()
            .key_context(BROWSER_CONTEXT)
            .flex()
            .flex_1()
            .min_w_0()
            .h_full();
        Self::with_actions(root, cx).child(Sheet {
            tabs: self.tab_strip_model(cx),
            toolbar,
            body: pane.into_any_element(),
            item_count,
            selected_count,
        })
    }
}

impl Focusable for BrowserView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_pane().focus_handle(cx)
    }
}
