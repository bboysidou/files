use std::path::PathBuf;
use std::rc::Rc;

use gpui::{App, Context, SharedString};

use crate::core::helpers::files::display_name::display_name;
use crate::features::browser::presentation::state::drop_request::DropRequest;
use crate::features::browser::presentation::state::pins::Pins;
use crate::features::network::domain::usecases::network_mount_root::network_mount_root;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::device_item::DeviceItem;
use crate::features::shell::presentation::state::favorite_rail::FavoriteRail;
use crate::global_components::handlers::{IndexHandler, IndexPathsHandler};

impl ShellView {
    pub(super) fn favorite_paths(cx: &App) -> Vec<PathBuf> {
        cx.try_global::<Pins>()
            .map(|pins| pins.0.favorites.clone())
            .unwrap_or_default()
    }

    fn favorite_label(path: &PathBuf) -> String {
        match network_mount_root(path) {
            Some((mount, label)) if mount == *path => label,
            _ => display_name(path),
        }
    }

    pub(super) fn favorite_items(location: &PlaceTarget, cx: &App) -> Vec<DeviceItem> {
        Self::favorite_paths(cx)
            .into_iter()
            .map(|path| DeviceItem {
                label: SharedString::from(Self::favorite_label(&path)),
                active: matches!(location, PlaceTarget::Directory(current) if *current == path),
                mounted: false,
                saved: true,
                protocol: None,
            })
            .collect()
    }

    pub(super) fn open_favorite(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(path) = Self::favorite_paths(cx).into_iter().nth(index) else {
            return;
        };
        self.browser.update(cx, |browser, cx| {
            browser.navigate(PlaceTarget::Directory(path), cx)
        });
    }

    pub(super) fn remove_favorite(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(path) = Self::favorite_paths(cx).into_iter().nth(index) else {
            return;
        };
        cx.global_mut::<Pins>().0.toggle_favorite(path);
        self.persist(cx);
        cx.refresh_windows();
        cx.notify();
    }

    fn favorite_drop_handler(&self) -> IndexPathsHandler {
        let browser = self.browser.downgrade();
        Rc::new(move |index, sources, window, cx| {
            let Some(destination) = Self::favorite_paths(cx).into_iter().nth(index) else {
                return;
            };
            let request = DropRequest {
                sources,
                destination,
                copy: window.modifiers().control,
            };
            let _ = browser.update(cx, |browser, cx| browser.drop_files(request, cx));
        })
    }

    pub(super) fn favorite_rail(
        &self,
        location: &PlaceTarget,
        cx: &mut Context<Self>,
    ) -> FavoriteRail {
        let (open_view, remove_view) = (cx.entity().downgrade(), cx.entity().downgrade());
        let on_select: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = open_view.update(cx, |this, cx| this.open_favorite(index, cx));
        });
        let on_remove: IndexHandler = Rc::new(move |index, _, cx| {
            cx.stop_propagation();
            let _ = remove_view.update(cx, |this, cx| this.remove_favorite(index, cx));
        });
        FavoriteRail {
            items: Self::favorite_items(location, cx),
            on_select,
            on_remove,
            on_drop: self.favorite_drop_handler(),
        }
    }
}
