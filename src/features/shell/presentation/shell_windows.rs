use gpui::{
    App, AppContext, Bounds, Context, Div, Focusable, InteractiveElement, Window, WindowBounds,
    WindowOptions, px, size,
};

use crate::core::assets::app_icon::app_icon;
use crate::core::constants::app::{APP_ID, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::core::dependency_injection::places::places_repository;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::integration::domain::entities::launch_request::LaunchRequest;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::shell::presentation::actions::{DetachTab, NewWindow};
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::shell_start::{ShellStart, WindowRole};

pub fn open_shell_window(start: ShellStart, cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)), cx);
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        app_id: Some(APP_ID.into()),
        icon: app_icon(),
        ..Default::default()
    };
    let opened = cx.open_window(options, |window, cx| {
        let view = cx.new(|cx| ShellView::new(places_repository().as_ref(), start, window, cx));
        window.focus(&view.focus_handle(cx), cx);
        view
    });
    if opened.is_ok() {
        cx.activate(true);
    }
}

impl ShellView {
    pub(super) fn with_window_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(
                cx.listener(|this, _: &NewWindow, window, cx| this.new_window(false, window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &DetachTab, window, cx| this.new_window(true, window, cx)),
            )
    }

    fn new_window(&mut self, detach: bool, window: &mut Window, cx: &mut Context<Self>) {
        if detach {
            return self
                .browser
                .update(cx, |browser, cx| browser.tear_active(window, cx));
        }
        let launch = match self.browser.read(cx).location(cx) {
            PlaceTarget::Directory(path) => LaunchRequest::Open(path),
            _ => LaunchRequest::Window,
        };
        let start = ShellStart {
            launch,
            role: WindowRole::Extra,
            tabs: None,
        };
        cx.defer(move |cx| open_shell_window(start, cx));
    }

    pub(super) fn open_detached(tab: TabSnapshot, cx: &mut Context<Self>) {
        let start = ShellStart {
            launch: LaunchRequest::Window,
            role: WindowRole::Extra,
            tabs: Some((vec![tab], 0)),
        };
        cx.defer(move |cx| open_shell_window(start, cx));
    }
}
