use std::rc::Rc;

use gpui::{AnyElement, IntoElement, SharedString, WeakEntity};
use rust_i18n::t;

use crate::core::enums::browser::sort_key::SortKey;
use crate::core::enums::view::view_mode::ViewMode;
use crate::features::browser::domain::entities::list_columns::ListColumns;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::setting_change::SettingChange;
use crate::features::browser::presentation::state::setting_item::{SettingControl, SettingItem};
use crate::global_components::choice_group::ChoiceGroup;
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::toggle_switch::ToggleSwitch;

const VIEW_KEYS: [&str; 4] = [
    "settings.view.list",
    "settings.view.columns",
    "settings.view.grid",
    "settings.view.dual",
];
const SORT_KEYS: [&str; 6] = [
    "settings.sort.name",
    "settings.sort.size",
    "settings.sort.kind",
    "settings.sort.modified",
    "settings.sort.extension",
    "settings.sort.created",
];
const COLUMN_LABELS: [(&str, &str); 4] = [
    ("set-col-size", "settings.column.size"),
    ("set-col-kind", "settings.column.kind"),
    ("set-col-modified", "settings.column.modified"),
    ("set-col-created", "settings.column.created"),
];
const START_KEYS: [&str; 2] = ["settings.start.last", "settings.start.home"];

impl BrowserView {
    pub(super) fn list_items(&self) -> Vec<SettingItem> {
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

    pub(super) fn layout_items(&self) -> Vec<SettingItem> {
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
            toggle(
                "set-folder-views",
                "settings.folder_views",
                self.settings.folder_views.enabled,
                SettingChange::FolderViews,
            ),
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

    pub(super) fn column_items(&self) -> Vec<SettingItem> {
        COLUMN_LABELS
            .iter()
            .zip(ListColumns::KEYS)
            .map(|((id, label), key)| {
                let on = self.options.columns.width_of(key).is_some();
                SettingItem {
                    id,
                    label,
                    control: SettingControl::Toggle {
                        on,
                        change: SettingChange::Column(key, !on),
                    },
                }
            })
            .collect()
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

pub(super) fn setting_control(item: SettingItem, view: WeakEntity<BrowserView>) -> AnyElement {
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
