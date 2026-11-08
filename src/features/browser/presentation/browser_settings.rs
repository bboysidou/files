use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, Context, Focusable, FontWeight, IntoElement, MouseButton,
    Pixels, SharedString, Size, WeakEntity, Window, anchored, deferred, div, point, prelude::*, px,
    rgb,
};
use rust_i18n::t;

use crate::core::enums::browser::sort_key::SortKey;
use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::colors::{CARD, FG_BRIGHT, LINE_STRONG};
use crate::core::theme::metrics::{
    QUICK_LOOK_MARGIN, QUICK_LOOK_RADIUS, SETTINGS_PANEL_HEIGHT, SETTINGS_PANEL_WIDTH,
    SHEET_PANEL_PAD, TEXT_EMPTY_TITLE, TOOLBAR_GAP,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::settings_row::SettingsRow;
use crate::features::browser::presentation::key_bindings::SETTINGS_CONTEXT;
use crate::features::browser::presentation::state::setting_change::SettingChange;
use crate::features::browser::presentation::state::setting_item::{SettingControl, SettingItem};
use crate::global_components::choice_group::ChoiceGroup;
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::modal_scrim::ModalScrim;
use crate::global_components::toggle_switch::ToggleSwitch;

const SETTINGS_PRIORITY: usize = 1;
const DOUBLE: f32 = 2.0;
const VIEW_KEYS: [&str; 4] = [
    "settings.view.list",
    "settings.view.columns",
    "settings.view.grid",
    "settings.view.dual",
];
const SORT_KEYS: [&str; 5] = [
    "settings.sort.name",
    "settings.sort.size",
    "settings.sort.kind",
    "settings.sort.modified",
    "settings.sort.extension",
];
const START_KEYS: [&str; 2] = ["settings.start.last", "settings.start.home"];

impl BrowserView {
    pub(super) fn open_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings.open = true;
        window.focus(&self.settings.focus, cx);
        cx.notify();
    }

    pub(super) fn close_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings.open = false;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn change_setting(&mut self, change: SettingChange, cx: &mut Context<Self>) {
        let mut options = self.options;
        match change {
            SettingChange::ShowHidden(on) => options.show_hidden = on,
            SettingChange::HiddenLast(on) => options.sort.hidden_last = on,
            SettingChange::SortBy(key) => options.sort.key = key,
            SettingChange::Ascending(on) => options.sort.ascending = on,
            SettingChange::View(view) => options.view = view,
            SettingChange::PreviewColumn => return self.toggle_preview_column(cx),
            SettingChange::RestoreTabs(on) => {
                self.settings.restore_tabs = on;
                return cx.notify();
            }
        }
        self.apply_view_options(options, cx);
    }

    fn list_items(&self) -> Vec<SettingItem> {
        let sort = self.options.sort;
        vec![
            toggle(
                "set-hidden",
                "settings.hidden",
                self.options.show_hidden,
                SettingChange::ShowHidden,
            ),
            toggle(
                "set-hidden-last",
                "settings.hidden_last",
                sort.hidden_last,
                SettingChange::HiddenLast,
            ),
            SettingItem {
                id: "set-sort",
                label: "settings.sort",
                control: SettingControl::Choice {
                    labels: SORT_KEYS.to_vec(),
                    selected: SortKey::ALL
                        .iter()
                        .position(|key| *key == sort.key)
                        .unwrap_or_default(),
                    pick: |index| SortKey::ALL.get(index).copied().map(SettingChange::SortBy),
                },
            },
            toggle(
                "set-ascending",
                "settings.ascending",
                sort.ascending,
                SettingChange::Ascending,
            ),
        ]
    }

    fn layout_items(&self) -> Vec<SettingItem> {
        let view = self.options.view;
        vec![
            SettingItem {
                id: "set-view",
                label: "settings.view",
                control: SettingControl::Choice {
                    labels: VIEW_KEYS.to_vec(),
                    selected: ViewMode::ALL
                        .iter()
                        .position(|mode| *mode == view)
                        .unwrap_or_default(),
                    pick: |index| ViewMode::ALL.get(index).copied().map(SettingChange::View),
                },
            },
            SettingItem {
                id: "set-preview",
                label: "settings.preview",
                control: SettingControl::Toggle {
                    on: self.column_open,
                    change: SettingChange::PreviewColumn,
                },
            },
            SettingItem {
                id: "set-start",
                label: "settings.start",
                control: SettingControl::Choice {
                    labels: START_KEYS.to_vec(),
                    selected: usize::from(!self.settings.restore_tabs),
                    pick: |index| Some(SettingChange::RestoreTabs(index == 0)),
                },
            },
        ]
    }

    fn settings_rows(&self, cx: &Context<Self>) -> Vec<AnyElement> {
        let view = cx.entity().downgrade();
        let mut items = self.list_items();
        items.extend(self.layout_items());
        items
            .into_iter()
            .map(|item| {
                SettingsRow {
                    label: t!(item.label).to_string().into(),
                    control: setting_control(item, view.clone()),
                }
                .into_any_element()
            })
            .collect()
    }

    fn settings_panel(&self, viewport: Size<Pixels>, cx: &Context<Self>) -> AnyElement {
        let margin = px(QUICK_LOOK_MARGIN * DOUBLE);
        div()
            .flex()
            .flex_col()
            .gap(px(TOOLBAR_GAP))
            .w(px(SETTINGS_PANEL_WIDTH).min(viewport.width - margin))
            .max_h(px(SETTINGS_PANEL_HEIGHT).min(viewport.height - margin))
            .p(px(SHEET_PANEL_PAD))
            .rounded(px(QUICK_LOOK_RADIUS))
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(LINE_STRONG))
            .shadow_lg()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .text_size(px(TEXT_EMPTY_TITLE))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(FG_BRIGHT))
                    .child(t!("settings.title").to_string()),
            )
            .child(
                div()
                    .id("settings-rows")
                    .flex()
                    .flex_col()
                    .min_h_0()
                    .overflow_y_scroll()
                    .children(self.settings_rows(cx)),
            )
            .into_any_element()
    }

    pub(super) fn settings_overlay(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.settings.open {
            return None;
        }
        let viewport = window.viewport_size();
        let panel = self.settings_panel(viewport, cx);
        let view = cx.entity().downgrade();
        let on_dismiss: ClickHandler = Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.close_settings(window, cx));
        });
        let scrim = ModalScrim {
            focus: self.settings.focus.clone(),
            context: SETTINGS_CONTEXT,
            viewport,
            on_dismiss,
            panel,
        };
        let overlay = anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.0), px(0.0)))
            .child(scrim);
        Some(
            deferred(overlay)
                .with_priority(SETTINGS_PRIORITY)
                .into_any_element(),
        )
    }
}

fn toggle(
    id: &'static str,
    label: &'static str,
    on: bool,
    change: fn(bool) -> SettingChange,
) -> SettingItem {
    SettingItem {
        id,
        label,
        control: SettingControl::Toggle {
            on,
            change: change(!on),
        },
    }
}

fn setting_control(item: SettingItem, view: WeakEntity<BrowserView>) -> AnyElement {
    match item.control {
        SettingControl::Toggle { on, change } => {
            let on_toggle: ClickHandler = Rc::new(move |_, cx| {
                let _ = view.update(cx, |this, cx| this.change_setting(change, cx));
            });
            ToggleSwitch {
                id: item.id.into(),
                on,
                on_toggle,
            }
            .into_any_element()
        }
        SettingControl::Choice {
            labels,
            selected,
            pick,
        } => {
            let on_select: IndexHandler = Rc::new(move |index, _, cx| {
                if let Some(change) = pick(index) {
                    let _ = view.update(cx, |this, cx| this.change_setting(change, cx));
                }
            });
            ChoiceGroup {
                id: item.id,
                labels: labels
                    .iter()
                    .map(|key| SharedString::from(t!(*key).to_string()))
                    .collect(),
                selected,
                on_select,
            }
            .into_any_element()
        }
    }
}
