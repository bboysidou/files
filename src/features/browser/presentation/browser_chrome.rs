use std::rc::Rc;

use gpui::{AnyElement, Context, IntoElement, ParentElement, SharedString, Styled, div, px};
use rust_i18n::t;

use crate::core::theme::metrics::TOAST_OFFSET;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::handlers::ViewHandler;
use crate::features::browser::presentation::state::tab_strip_model::TabStripModel;
use crate::global_components::handlers::IndexPairHandler;
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::toast::Toast;

impl BrowserView {
    pub(super) fn tab_strip_model(&self, cx: &mut Context<Self>) -> TabStripModel {
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
        let move_view = cx.entity().downgrade();
        let on_move: IndexPairHandler = Rc::new(move |pair, window, cx| {
            let _ = move_view.update(cx, |this, cx| this.move_tab(pair, window, cx));
        });
        TabStripModel {
            titles: self
                .tabs
                .iter()
                .map(|tab| tab.pane.read(cx).title(cx))
                .collect(),
            active: self.active,
            on_select,
            on_close,
            on_move,
            on_new,
        }
    }

    pub(super) fn toast_element(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let toast = self.toasts.current.clone()?;
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

    pub(super) fn chrome_handlers(
        cx: &mut Context<Self>,
    ) -> (ClickHandler, ClickHandler, ViewHandler, ClickHandler) {
        let (preview_view, keys_view, mode_view) = (
            cx.entity().downgrade(),
            cx.entity().downgrade(),
            cx.entity().downgrade(),
        );
        let on_preview: ClickHandler = Rc::new(move |_, cx| {
            let _ = preview_view.update(cx, |this, cx| this.toggle_preview_column(cx));
        });
        let on_keys: ClickHandler = Rc::new(move |window, cx| {
            let _ = keys_view.update(cx, |this, cx| this.open_keymap(window, cx));
        });
        let on_view: ViewHandler = Rc::new(move |view, _, cx| {
            let _ = mode_view.update(cx, |this, cx| this.set_view(view, cx));
        });
        let settings_view = cx.entity().downgrade();
        let on_settings: ClickHandler = Rc::new(move |window, cx| {
            let _ = settings_view.update(cx, |this, cx| this.open_settings(window, cx));
        });
        (on_preview, on_keys, on_view, on_settings)
    }
}
