use std::path::Path;

use gpui::Context;

use crate::core::enums::view::view_mode::ViewMode;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::browser_view::BrowserView;

impl BrowserView {
    pub(super) fn remember_view(&mut self, view: ViewMode, cx: &mut Context<Self>) {
        if let Some(folder) = self.active_pane().read(cx).directory() {
            self.settings.folder_views.record(&folder, view);
        }
    }

    pub(super) fn recall_folder_view(&mut self, folder: &Path, cx: &mut Context<Self>) {
        if !self.settings.folder_views.enabled {
            return;
        }
        let Some(view) = self.settings.folder_views.get(folder) else {
            return;
        };
        if view != self.options.view {
            let options = ViewOptions {
                view,
                ..self.options
            };
            self.apply_view_options(options, cx);
        }
    }
}
