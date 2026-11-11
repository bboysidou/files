use std::ops::Range;
use std::rc::Rc;

use gpui::{AnyElement, Context, SharedString, div, prelude::*, px, rgb, uniform_list};
use rust_i18n::t;

use crate::core::enums::network::server_source::ServerSource;
use crate::core::enums::network::server_tab::ServerTab;
use crate::core::theme::colors::{ACCENT, FG_BRIGHT, FG_MUTED};
use crate::core::theme::metrics::{
    CONTROL_RADIUS, CRUMBS_HEIGHT, FILTER_FIELD_PAD_X, SERVER_LIST_HEIGHT,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::server_row::ServerRow;
use crate::features::browser::presentation::key_bindings::SERVER_LIST_CONTEXT;
use crate::features::browser::presentation::state::server_edit_state::ServerEditState;
use crate::global_components::choice_group::ChoiceGroup;
use crate::global_components::handlers::IndexHandler;

const TAB_CHOICE_ID: &str = "server-tab";
const LIST_ID: &str = "server-list";

impl BrowserView {
    pub(super) fn server_tabs(
        edit: &ServerEditState,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if edit.original.is_some() {
            return None;
        }
        let view = cx.entity().downgrade();
        let on_select: IndexHandler = Rc::new(move |index, window, cx| {
            if let Some(tab) = ServerTab::ALL.get(index).copied() {
                let _ = view.update(cx, |this, cx| this.switch_server_tab(tab, window, cx));
            }
        });
        Some(
            ChoiceGroup {
                id: TAB_CHOICE_ID,
                labels: ServerTab::ALL
                    .iter()
                    .map(|tab| SharedString::from(t!(tab.label_key()).to_string()))
                    .collect(),
                selected: edit.tab.index(),
                on_select,
            }
            .into_any_element(),
        )
    }

    fn server_row_handlers(cx: &mut Context<Self>) -> [IndexHandler; 3] {
        let views = [(); 3].map(|_| cx.entity().downgrade());
        let [connect_view, pin_view, forget_view] = views;
        let on_connect: IndexHandler = Rc::new(move |index, window, cx| {
            let _ = connect_view.update(cx, |this, cx| {
                this.connect_server_choice(Some(index), window, cx)
            });
        });
        let on_pin: IndexHandler = Rc::new(move |index, _, cx| {
            cx.stop_propagation();
            let _ = pin_view.update(cx, |this, cx| this.toggle_server_pin(Some(index), cx));
        });
        let on_forget: IndexHandler = Rc::new(move |index, _, cx| {
            cx.stop_propagation();
            let _ = forget_view.update(cx, |this, cx| this.forget_server_choice(Some(index), cx));
        });
        [on_connect, on_pin, on_forget]
    }

    fn server_rows(&mut self, range: Range<usize>, cx: &mut Context<Self>) -> Vec<ServerRow> {
        let Some(edit) = self.server_edit.as_ref() else {
            return Vec::new();
        };
        let cursor = edit.list.cursor;
        let choices = Self::server_choice_list(edit, cx);
        let [on_connect, on_pin, on_forget] = Self::server_row_handlers(cx);
        choices
            .into_iter()
            .enumerate()
            .skip(range.start)
            .take(range.len())
            .map(|(index, choice)| ServerRow {
                index,
                selected: index == cursor,
                on_forget: (choice.source == ServerSource::Saved).then(|| on_forget.clone()),
                choice,
                on_connect: on_connect.clone(),
                on_pin: on_pin.clone(),
            })
            .collect()
    }

    pub(super) fn server_list_view(
        edit: &ServerEditState,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let count = Self::server_choice_list(edit, cx).len();
        let filter = div()
            .flex()
            .items_center()
            .h(px(CRUMBS_HEIGHT))
            .px(px(FILTER_FIELD_PAD_X))
            .rounded(px(CONTROL_RADIUS))
            .border_1()
            .border_color(rgb(ACCENT))
            .text_color(rgb(FG_BRIGHT))
            .key_context(SERVER_LIST_CONTEXT)
            .child(edit.list.filter.clone())
            .into_any_element();
        let body = if count == 0 {
            let key = if edit.list.filter.read(cx).text().is_empty() {
                "server_edit.list_empty"
            } else {
                "server_edit.no_match"
            };
            div()
                .h(px(SERVER_LIST_HEIGHT))
                .flex()
                .items_center()
                .justify_center()
                .text_color(rgb(FG_MUTED))
                .child(t!(key).to_string())
                .into_any_element()
        } else {
            uniform_list(
                LIST_ID,
                count,
                cx.processor(|this, range: Range<usize>, _window, cx| this.server_rows(range, cx)),
            )
            .h(px(SERVER_LIST_HEIGHT))
            .track_scroll(&edit.list.scroll)
            .into_any_element()
        };
        let hint = div()
            .text_color(rgb(FG_MUTED))
            .child(t!("server_edit.list_hint").to_string())
            .into_any_element();
        vec![filter, body, hint]
    }
}
