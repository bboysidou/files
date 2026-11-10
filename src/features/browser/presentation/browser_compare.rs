use std::path::PathBuf;

use gpui::{Context, Entity, Focusable, Window, point, px};
use rust_i18n::t;

use crate::core::theme::metrics::QUICK_LOOK_SCROLL_STEP;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::compare_state::CompareState;

impl BrowserView {
    fn compare_files_of(&self, pane: &Entity<PaneView>, cx: &Context<Self>) -> Vec<PathBuf> {
        let files = &self.setup.operations;
        pane.read(cx)
            .target_paths()
            .into_iter()
            .filter(|path| !files.is_dir(path))
            .collect()
    }

    fn compare_targets(&self, cx: &Context<Self>) -> Option<(PathBuf, PathBuf)> {
        let mine = self.compare_files_of(self.active_pane(), cx);
        if let [left, right] = mine.as_slice() {
            return Some((left.clone(), right.clone()));
        }
        let other = self.compare_files_of(&self.other_pane()?, cx);
        match (mine.as_slice(), other.as_slice()) {
            ([mine], [other]) if self.dual.focused => Some((other.clone(), mine.clone())),
            ([mine], [other]) => Some((mine.clone(), other.clone())),
            _ => None,
        }
    }

    pub(super) fn open_compare(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((left, right)) = self.compare_targets(cx) else {
            return self.show_toast(t!("compare.need_two").to_string(), false, cx);
        };
        self.compare.state = Some(CompareState {
            left: left.clone(),
            right: right.clone(),
            result: None,
        });
        window.focus(&self.compare.focus, cx);
        let comparer = self.setup.comparer.clone();
        self.compare.task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { comparer.compare(&left, &right) })
                .await;
            let _ = this.update(cx, |this, cx| {
                if let Some(state) = this.compare.state.as_mut() {
                    state.result = Some(result);
                    cx.notify();
                }
            });
        }));
        cx.notify();
    }

    pub(super) fn close_compare(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.compare.state = None;
        self.compare.task = None;
        window.focus(&self.active_pane().focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn scroll_compare(&mut self, direction: f32, cx: &mut Context<Self>) {
        let handle = self.compare.scroll.0.borrow().base_handle.clone();
        let (offset, max) = (handle.offset(), handle.max_offset());
        let target = offset.y - px(QUICK_LOOK_SCROLL_STEP * direction);
        handle.set_offset(point(offset.x, target.clamp(-max.y, px(0.0))));
        cx.notify();
    }
}
