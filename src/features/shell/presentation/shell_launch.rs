use std::path::Path;
use std::rc::Rc;

use gpui::{App, Context, Window};

use crate::features::integration::domain::entities::launch_request::LaunchRequest;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::shell_windows::open_shell_window;
use crate::features::shell::presentation::state::open_shells::{OpenShell, OpenShells};
use crate::features::shell::presentation::state::saved_tab_mapping::restored_tab;
use crate::features::shell::presentation::state::shell_start::{ShellStart, WindowRole};
use crate::global_components::drag::marker_drop::MarkerDrop;

impl ShellView {
    pub(super) fn start(
        &mut self,
        (launch, restored): (LaunchRequest, bool),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.register_shell(window, cx);
        self.refresh_devices(cx);
        self.watch_servers(cx);
        self.watch_devices(cx);
        self.watch_network(cx);
        self.watch_phones(cx);
        if self.role == WindowRole::Primary && self.saved.restore_tabs {
            self.reopen_windows(cx);
        }
        self.handle_launch(launch, window, cx);
        if !restored && (self.role == WindowRole::Extra || !self.saved.restore_tabs) {
            self.browser
                .update(cx, |browser, cx| browser.drop_start_placeholder(window, cx));
        }
    }

    fn reopen_windows(&self, cx: &mut Context<Self>) {
        for saved in self.saved.windows.clone() {
            let tabs: Vec<_> = saved.tabs.iter().filter_map(restored_tab).collect();
            if tabs.is_empty() {
                continue;
            }
            let start = ShellStart {
                launch: LaunchRequest::Window,
                role: WindowRole::Extra,
                tabs: Some((tabs, saved.active_tab)),
            };
            cx.defer(move |cx| open_shell_window(start, cx));
        }
    }

    fn register_shell(&self, window: &mut Window, cx: &mut Context<Self>) {
        let shell = OpenShell {
            view: cx.entity().downgrade(),
            window: window.window_handle(),
        };
        cx.default_global::<OpenShells>().0.push(shell);
        if !cx.has_global::<MarkerDrop>() {
            cx.set_global(MarkerDrop(Rc::new(adopt_dropped_tab)));
        }
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

fn adopt_dropped_tab(token: u64, window: &mut Window, cx: &mut App) {
    let handle = window.window_handle();
    let Some(browser) = cx.try_global::<OpenShells>().and_then(|shells| {
        shells
            .0
            .iter()
            .find(|shell| shell.window == handle)
            .and_then(|shell| shell.view.upgrade())
            .map(|view| view.read(cx).browser.clone())
    }) else {
        return;
    };
    browser.update(cx, |browser, cx| browser.adopt_tab(token, window, cx));
}
