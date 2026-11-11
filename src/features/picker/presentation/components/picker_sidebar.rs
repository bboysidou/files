use std::path::PathBuf;
use std::rc::Rc;

use gpui::{AnyElement, App, IntoElement, RenderOnce, Window, div, prelude::*, px};

use crate::core::theme::metrics::{RAIL_PAD_BOTTOM, RAIL_PAD_TOP, RAIL_PAD_X, RAIL_WIDTH};
use crate::features::picker::domain::entities::picker_shortcut::{
    PickerShortcut, ShortcutGroup, ShortcutLook,
};
use crate::global_components::handlers::IndexHandler;
use crate::global_components::rail::rail_device_item::RailDeviceItem;
use crate::global_components::rail::rail_motion::RailMotion;
use crate::global_components::rail::rail_nav_item::RailNavItem;
use crate::global_components::rail::rail_section_label::RailSectionLabel;

const SHORTCUT_ID: &str = "picker-shortcut";

#[derive(IntoElement)]
pub struct PickerSidebar {
    pub shortcuts: Vec<PickerShortcut>,
    pub active: Option<PathBuf>,
    pub on_select: IndexHandler,
}

impl PickerSidebar {
    fn row(&self, index: usize, shortcut: &PickerShortcut, motion: RailMotion) -> AnyElement {
        let active = self.active.as_ref() == Some(&shortcut.path);
        match &shortcut.look {
            ShortcutLook::Place(kind) => RailNavItem {
                index,
                kind: *kind,
                active,
                cursor: false,
                motion,
                on_select: self.on_select.clone(),
                on_drop: None,
                on_context: None,
            }
            .into_any_element(),
            ShortcutLook::Named(label) => RailDeviceItem {
                index,
                id: SHORTCUT_ID,
                icon: shortcut.group.icon(),
                label: label.clone().into(),
                trailing: None,
                protocol: None,
                active,
                cursor: false,
                motion,
                on_select: self.on_select.clone(),
                on_eject: Rc::new(|_, _| {}),
                on_edit: None,
                on_context: None,
                on_drop: None,
            }
            .into_any_element(),
        }
    }

    fn group(&self, group: ShortcutGroup, motion: RailMotion) -> Vec<AnyElement> {
        let rows: Vec<AnyElement> = self
            .shortcuts
            .iter()
            .enumerate()
            .filter(|(_, shortcut)| shortcut.group == group)
            .map(|(index, shortcut)| self.row(index, shortcut, motion))
            .collect();
        if rows.is_empty() {
            return rows;
        }
        let label = RailSectionLabel::new(group.label_key(), motion).into_any_element();
        std::iter::once(label).chain(rows).collect()
    }
}

impl RenderOnce for PickerSidebar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let motion = RailMotion::starting(true);
        let rows: Vec<AnyElement> = ShortcutGroup::ALL
            .iter()
            .flat_map(|group| self.group(*group, motion))
            .collect();
        div()
            .id("picker-sidebar")
            .flex()
            .flex_col()
            .flex_shrink_0()
            .h_full()
            .w(px(RAIL_WIDTH))
            .overflow_y_scroll()
            .pt(px(RAIL_PAD_TOP))
            .pb(px(RAIL_PAD_BOTTOM))
            .px(px(RAIL_PAD_X))
            .children(rows)
    }
}
