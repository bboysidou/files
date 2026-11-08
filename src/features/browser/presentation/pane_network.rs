use gpui::Context;
use rust_i18n::t;

use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::network::domain::entities::network_credentials::NetworkCredentials;
use crate::features::network::domain::entities::network_mount_error::NetworkMountError;
use crate::features::network::domain::usecases::uri_with_user::uri_with_user;
use crate::features::places::domain::entities::place::PlaceTarget;

impl PaneView {
    pub fn connect_network(
        &mut self,
        uri: String,
        credentials: Option<NetworkCredentials>,
        cx: &mut Context<Self>,
    ) {
        cx.emit(PaneEvent::Message(
            t!("network.connecting", uri = uri).to_string(),
        ));
        let mounter = self.setup.network.clone();
        let saved = uri_with_user(
            &uri,
            credentials.as_ref().map_or("", |given| given.user.as_str()),
        );
        cx.spawn(async move |this, cx| {
            let target = uri.clone();
            let mounted = cx
                .background_executor()
                .spawn(async move { mounter.mount(&target, credentials.as_ref()) })
                .await;
            let _ = this.update(cx, |this, cx| match mounted {
                Ok(path) => {
                    let edit = this.server_edit.pending.take();
                    cx.emit(PaneEvent::Connected { uri: saved, edit });
                    this.navigate(PlaceTarget::Directory(path), cx)
                }
                Err(NetworkMountError::NeedsCredentials) => {
                    cx.emit(PaneEvent::CredentialsRequested {
                        uri,
                        rejected: false,
                    })
                }
                Err(NetworkMountError::Rejected) => cx.emit(PaneEvent::CredentialsRequested {
                    uri,
                    rejected: true,
                }),
                Err(NetworkMountError::TimedOut) => {
                    this.server_edit.pending = None;
                    cx.emit(PaneEvent::Message(t!("network.timed_out").to_string()))
                }
                Err(NetworkMountError::Failed(reason)) => {
                    this.server_edit.pending = None;
                    cx.emit(PaneEvent::Message(
                        t!("network.failed", reason = reason).to_string(),
                    ))
                }
            });
        })
        .detach();
    }
}
