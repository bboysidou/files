use std::path::Path;

use futures::StreamExt;
use futures::channel::mpsc;
use gpui::{Context, Window};

use crate::core::dependency_injection::integration::{default_handler, instance_bridge};
use crate::features::integration::domain::entities::launch_request::LaunchRequest;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::shell_view::ShellView;

impl ShellView {
    pub(super) fn start(
        &mut self,
        launch: LaunchRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.refresh_devices(cx);
        self.watch_devices(cx);
        self.serve_launches(window, cx);
        self.handle_launch(launch, window, cx);
        self.browser
            .update(cx, |browser, cx| browser.drop_start_placeholder(window, cx));
    }

    fn serve_launches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (sender, mut requests) = mpsc::unbounded();
        let claim = default_handler().is_default();
        self.bus = instance_bridge().serve(sender, claim);
        self.launch_task = Some(cx.spawn_in(window, async move |this, cx| {
            while let Some(request) = requests.next().await {
                let handled = this.update_in(cx, |this, window, cx| {
                    window.activate_window();
                    this.handle_launch(request, window, cx);
                });
                if handled.is_err() {
                    break;
                }
            }
        }));
    }

    pub(super) fn handle_launch(
        &mut self,
        request: LaunchRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (folder, focus) = match request {
            LaunchRequest::Open(path) if path.is_dir() => (path, None),
            LaunchRequest::Open(path) | LaunchRequest::Reveal(path) if path.exists() => {
                let Some(parent) = path.parent().map(Path::to_path_buf) else {
                    return;
                };
                (parent, Some(path))
            }
            _ => return,
        };
        self.browser.update(cx, |browser, cx| {
            browser.open_tab((PlaceTarget::Directory(folder), focus), window, cx)
        });
    }
}
