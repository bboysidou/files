use std::path::PathBuf;

use gpui::Context;
use rust_i18n::t;

use crate::core::helpers::files::display_name::display_name;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::pins::Pins;

impl BrowserView {
    pub(super) fn favorite_target(&self, cx: &Context<Self>) -> Option<PathBuf> {
        let pane = self.active_pane().read(cx);
        pane.current_entry()
            .filter(|entry| entry.is_dir)
            .map(|entry| entry.path)
            .or_else(|| pane.directory())
    }

    pub(super) fn is_favorite_target(&self, cx: &Context<Self>) -> bool {
        let pins = cx.try_global::<Pins>();
        self.favorite_target(cx)
            .is_some_and(|path| pins.is_some_and(|pins| pins.0.is_favorite(&path)))
    }

    pub(super) fn toggle_favorite(&mut self, cx: &mut Context<Self>) {
        let Some(path) = self.favorite_target(cx) else {
            return;
        };
        let pins = &mut cx.global_mut::<Pins>().0;
        let key = if pins.is_favorite(&path) {
            "favorites.removed"
        } else {
            "favorites.added"
        };
        pins.toggle_favorite(path.clone());
        self.show_toast(t!(key, name = display_name(&path)).to_string(), false, cx);
        cx.refresh_windows();
        cx.notify();
    }
}
