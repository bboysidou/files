use std::time::Duration;

use futures::StreamExt;
use gpui::{Context, Task};

use crate::features::browser::domain::entities::watch_subscription::WatchSubscription;
use crate::features::shell::presentation::shell_view::ShellView;

pub(super) type ShellRefresh = fn(&mut ShellView, &mut Context<ShellView>);

impl ShellView {
    pub(super) fn debounced_refresh(
        subscription: WatchSubscription,
        debounce_ms: u64,
        refresh: ShellRefresh,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let (mut changes, guard) = (subscription.changes, subscription.guard);
        cx.spawn(async move |this, cx| {
            let _guard = guard;
            while changes.next().await.is_some() {
                cx.background_executor()
                    .timer(Duration::from_millis(debounce_ms))
                    .await;
                while changes.try_recv().is_ok() {}
                if this.update(cx, refresh).is_err() {
                    break;
                }
            }
        })
    }
}
