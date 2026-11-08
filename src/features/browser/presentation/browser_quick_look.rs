use gpui::{Context, Focusable, UniformListScrollHandle, Window};

use crate::core::constants::preview::{ZOOM_DEFAULT, ZOOM_MAX, ZOOM_MIN};
use crate::features::browser::domain::entities::file_entry::FileEntry;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::quick_look_state::QuickLookState;

impl BrowserView {
    pub(super) fn open_quick_look(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self.active_pane().read(cx).current_entry() else {
            return;
        };
        window.focus(&self.quick_look_focus, cx);
        self.show_preview(entry, cx);
    }

    pub(super) fn close_quick_look(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.quick_look = None;
        self.quick_look_task = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn step_quick_look(&mut self, delta: isize, cx: &mut Context<Self>) {
        let pane = self.active_pane().clone();
        pane.update(cx, |pane, cx| pane.move_cursor(delta, cx));
        let Some(entry) = pane.read(cx).current_entry() else {
            return;
        };
        let unchanged = self
            .quick_look
            .as_ref()
            .is_some_and(|state| state.entry.path == entry.path);
        if !unchanged {
            self.show_preview(entry, cx);
        }
    }

    pub(super) fn zoom_quick_look(&mut self, factor: f32, cx: &mut Context<Self>) {
        if let Some(state) = self.quick_look.as_mut() {
            state.zoom = (state.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
            cx.notify();
        }
    }

    fn show_preview(&mut self, entry: FileEntry, cx: &mut Context<Self>) {
        self.quick_look = Some(QuickLookState {
            entry: entry.clone(),
            content: None,
            zoom: ZOOM_DEFAULT,
        });
        self.preview_scroll = UniformListScrollHandle::new();
        self.quick_look_generation += 1;
        let generation = self.quick_look_generation;
        let source = self.setup.preview.clone();
        self.quick_look_task = Some(cx.spawn(async move |this, cx| {
            let content = cx
                .background_executor()
                .spawn(async move { source.load(&entry) })
                .await;
            let _ = this.update(cx, |this, cx| {
                if this.quick_look_generation != generation {
                    return;
                }
                if let Some(state) = this.quick_look.as_mut() {
                    state.content = Some(content);
                    cx.notify();
                }
            });
        }));
        cx.notify();
    }
}
