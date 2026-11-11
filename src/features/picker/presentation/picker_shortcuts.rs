use std::rc::Rc;

use gpui::Context;

use crate::core::dependency_injection::devices::device_repository;
use crate::core::dependency_injection::places::places_repository;
use crate::core::dependency_injection::settings::settings_repository;
use crate::features::picker::domain::entities::picker_shortcut::{PickerShortcut, ShortcutGroup};
use crate::features::picker::domain::usecases::device_shortcuts::device_shortcuts;
use crate::features::picker::domain::usecases::fixed_shortcuts::fixed_shortcuts;
use crate::features::picker::presentation::components::picker_sidebar::PickerSidebar;
use crate::features::picker::presentation::picker_view::PickerView;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::IndexHandler;

impl PickerView {
    pub(super) fn initial_shortcuts() -> Vec<PickerShortcut> {
        let favorites = settings_repository().load().pins.favorites;
        fixed_shortcuts(places_repository().places(), favorites)
    }

    pub(super) fn load_devices(&mut self, cx: &mut Context<Self>) {
        let repository = device_repository();
        self.devices_task = Some(cx.spawn(async move |this, cx| {
            let devices = cx
                .background_executor()
                .spawn(async move { device_shortcuts(repository.as_ref()) })
                .await;
            let _ = this.update(cx, |this, cx| {
                this.shortcuts
                    .retain(|shortcut| shortcut.group != ShortcutGroup::Devices);
                this.shortcuts.extend(devices);
                cx.notify();
            });
        }));
    }

    fn open_shortcut(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(shortcut) = self.shortcuts.get(index) else {
            return;
        };
        let target = PlaceTarget::Directory(shortcut.path.clone());
        self.pane.update(cx, |pane, cx| pane.navigate(target, cx));
    }

    pub(super) fn sidebar(&self, cx: &mut Context<Self>) -> PickerSidebar {
        let view = cx.entity().downgrade();
        let on_select: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = view.update(cx, |this, cx| this.open_shortcut(index, cx));
        });
        PickerSidebar {
            shortcuts: self.shortcuts.clone(),
            active: self.pane.read(cx).directory(),
            on_select,
        }
    }
}
