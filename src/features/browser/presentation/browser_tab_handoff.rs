use std::fs;
use std::path::PathBuf;
use std::process;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use gpui::{App, Context, Window};

use crate::core::constants::files::TEAR_MIN_TABS;
use crate::core::helpers::files::tab_marker_path::tab_marker_path;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::pending_tab::{PendingTab, PendingTabSlot};
use crate::global_components::handlers::{ClickHandler, IndexPathHandler};

static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);

fn take_pending(cx: &mut App) -> Option<PendingTab> {
    cx.try_global::<PendingTabSlot>()?;
    cx.global_mut::<PendingTabSlot>().0.take()
}

fn write_marker(token: u64) -> Option<PathBuf> {
    let marker = tab_marker_path(token)?;
    fs::create_dir_all(marker.parent()?).ok()?;
    fs::write(&marker, []).ok()?;
    Some(marker)
}

impl BrowserView {
    fn park_tab(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<PathBuf> {
        if self.tabs.len() < TEAR_MIN_TABS || index >= self.tabs.len() {
            return None;
        }
        let token = (u64::from(process::id()) << 32) | NEXT_TOKEN.fetch_add(1, Ordering::Relaxed);
        let marker = write_marker(token)?;
        if let Some(stale) = take_pending(cx) {
            let _ = fs::remove_file(stale.marker);
        }
        let source = cx.entity().downgrade();
        cx.set_global(PendingTabSlot(Some(PendingTab {
            source,
            window: window.window_handle(),
            index,
            token,
            marker: marker.clone(),
            landed_home: false,
        })));
        Some(marker)
    }

    pub fn adopt_tab(&mut self, token: u64, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pending) = take_pending(cx).filter(|pending| pending.token == token) else {
            return;
        };
        let _ = fs::remove_file(&pending.marker);
        if pending.source == cx.entity().downgrade() {
            return;
        }
        let Some(source) = pending.source.upgrade() else {
            return;
        };
        let Some(snapshot) = source
            .read(cx)
            .tabs
            .get(pending.index)
            .map(|tab| tab.pane.read(cx).tab_snapshot())
        else {
            return;
        };
        let tab = self.open_pane(snapshot, window, cx);
        self.tabs.push(tab);
        self.activate(self.tabs.len() - 1, window, cx);
        let (handle, index) = (pending.window, pending.index);
        cx.defer(move |cx| {
            let _ = handle.update(cx, |_, window, cx| {
                source.update(cx, |browser, cx| browser.close_tab(index, window, cx));
            });
        });
    }

    fn finish_drag_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let me = cx.entity().downgrade();
        let ours = cx
            .try_global::<PendingTabSlot>()
            .and_then(|slot| slot.0.as_ref())
            .is_some_and(|pending| pending.source == me);
        if !ours {
            return;
        }
        let Some(pending) = take_pending(cx) else {
            return;
        };
        let _ = fs::remove_file(&pending.marker);
        if !pending.landed_home {
            self.tear_tab(pending.index, window, cx);
        }
    }

    fn mark_landed_home(&mut self, cx: &mut Context<Self>) {
        let me = cx.entity().downgrade();
        if let Some(pending) = cx
            .try_global::<PendingTabSlot>()
            .is_some()
            .then(|| cx.global_mut::<PendingTabSlot>())
            .and_then(|slot| slot.0.as_mut())
            .filter(|pending| pending.source == me)
        {
            pending.landed_home = true;
        }
    }

    pub(super) fn park_handler(cx: &mut Context<Self>) -> IndexPathHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |index, window, cx| {
            view.update(cx, |this, cx| this.park_tab(index, window, cx))
                .ok()
                .flatten()
        })
    }

    pub(super) fn drag_end_handler(cx: &mut Context<Self>) -> ClickHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| this.finish_drag_out(window, cx));
        })
    }

    pub(super) fn drag_home_handler(cx: &mut Context<Self>) -> ClickHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |_, cx| {
            let _ = view.update(cx, |this, cx| this.mark_landed_home(cx));
        })
    }
}
