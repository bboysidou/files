use gpui::Context;

use crate::core::enums::browser::sort_key::SortKey;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::pane_event::PaneEvent;

impl PaneView {
    pub(super) fn view_options(&self) -> ViewOptions {
        ViewOptions {
            show_hidden: self.show_hidden,
            sort: self.sort,
        }
    }

    pub(super) fn set_view_options(&mut self, options: ViewOptions, cx: &mut Context<Self>) {
        if options == self.view_options() {
            return;
        }
        self.show_hidden = options.show_hidden;
        self.sort = options.sort;
        self.rearrange(cx);
    }

    pub(super) fn set_compact(&mut self, compact: bool, cx: &mut Context<Self>) {
        if self.compact != compact {
            self.compact = compact;
            cx.notify();
        }
    }

    pub(super) fn toggle_hidden(&mut self, cx: &mut Context<Self>) {
        self.show_hidden = !self.show_hidden;
        self.rearrange(cx);
        cx.emit(PaneEvent::ViewOptionsChanged(self.view_options()));
    }

    pub(super) fn sort_by(&mut self, key: SortKey, cx: &mut Context<Self>) {
        self.sort = self.sort.toggled_on(key);
        self.rearrange(cx);
        cx.emit(PaneEvent::ViewOptionsChanged(self.view_options()));
    }
}
