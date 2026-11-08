use gpui::Context;
use rust_i18n::t;

use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::places::domain::entities::place::PlaceTarget;

impl PaneView {
    pub(super) fn connect_network(&mut self, uri: String, cx: &mut Context<Self>) {
        cx.emit(PaneEvent::Message(
            t!("network.connecting", uri = uri).to_string(),
        ));
        let mounter = self.setup.network.clone();
        cx.spawn(async move |this, cx| {
            let mounted = cx
                .background_executor()
                .spawn(async move { mounter.mount(&uri) })
                .await;
            let _ = this.update(cx, |this, cx| match mounted {
                Ok(path) => this.navigate(PlaceTarget::Directory(path), cx),
                Err(reason) => cx.emit(PaneEvent::Message(
                    t!("network.failed", reason = reason).to_string(),
                )),
            });
        })
        .detach();
    }
}
