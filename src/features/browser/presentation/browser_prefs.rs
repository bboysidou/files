use gpui::Context;

use crate::core::constants::settings::PERMILLE;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::browser_snapshot::BrowserSnapshot;

impl BrowserView {
    pub fn apply_prefs(&mut self, prefs: &BrowserSnapshot, cx: &mut Context<Self>) {
        let mine = self.snapshot(cx);
        if mine.preview_open != prefs.preview_open {
            self.toggle_preview_column(cx);
        }
        if mine.dual_split != prefs.dual_split {
            self.dual.split = f32::from(prefs.dual_split) / PERMILLE;
            cx.notify();
        }
        let settings_changed = mine.restore_tabs != prefs.restore_tabs
            || mine.key_preset != prefs.key_preset
            || mine.folder_views != prefs.folder_views;
        if settings_changed {
            self.settings.restore_tabs = prefs.restore_tabs;
            self.settings.key_preset = prefs.key_preset;
            self.settings.folder_views = prefs.folder_views.clone();
            cx.notify();
        }
        if mine.options != prefs.options {
            self.apply_view_options(prefs.options, cx);
        }
    }
}
