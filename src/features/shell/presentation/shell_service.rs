use std::time::Duration;

use futures::StreamExt;
use futures::channel::mpsc;
use gpui::{AnyWindowHandle, App, WeakEntity};

use crate::core::constants::preview::{
    ARCHIVE_BROWSE_DIR, CACHE_MAX_AGE_SECS, CACHE_MAX_BYTES, PREVIEW_MEDIA_DIR, PREVIEW_PDF_DIR,
};
use crate::core::dependency_injection::integration::{default_handler, instance_bridge};
use crate::core::helpers::files::prune_cache::prune_cache;
use crate::features::integration::domain::entities::launch_request::LaunchRequest;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::shell_windows::open_shell_window;
use crate::features::shell::presentation::state::launch_service::LaunchService;
use crate::features::shell::presentation::state::open_shells::{OpenShell, OpenShells};
use crate::features::shell::presentation::state::shell_start::{ShellStart, WindowRole};

pub fn serve_launches(cx: &mut App) {
    let (sender, mut requests) = mpsc::unbounded();
    let claim = default_handler().is_default();
    let bus = instance_bridge().serve(sender, claim);
    let task = cx.spawn(async move |cx| {
        while let Some(request) = requests.next().await {
            cx.update(|cx| route_launch(request, cx));
        }
    });
    cx.set_global(LaunchService {
        _bus: bus,
        _task: task,
    });
    cx.background_executor()
        .spawn(async { prune_caches() })
        .detach();
}

fn route_launch(request: LaunchRequest, cx: &mut App) {
    let Some((view, window)) = launch_target(cx) else {
        let start = ShellStart {
            launch: request,
            role: WindowRole::Extra,
            tabs: None,
        };
        return open_shell_window(start, cx);
    };
    let _ = window.update(cx, |_, window, cx| {
        window.activate_window();
        let _ = view.update(cx, |this, cx| this.handle_launch(request, window, cx));
    });
}

fn launch_target(cx: &App) -> Option<(WeakEntity<ShellView>, AnyWindowHandle)> {
    let shells = &cx.try_global::<OpenShells>()?.0;
    let live = |shell: &&OpenShell| shell.view.upgrade().is_some();
    let active = cx.active_window();
    shells
        .iter()
        .filter(live)
        .find(|shell| Some(shell.window) == active)
        .or_else(|| shells.iter().rev().find(live))
        .map(|shell| (shell.view.clone(), shell.window))
}

fn prune_caches() {
    let Some(cache) = dirs::cache_dir() else {
        return;
    };
    let max_age = Duration::from_secs(CACHE_MAX_AGE_SECS);
    for dir in [ARCHIVE_BROWSE_DIR, PREVIEW_PDF_DIR, PREVIEW_MEDIA_DIR] {
        prune_cache(&cache.join(dir), max_age, CACHE_MAX_BYTES);
    }
}
