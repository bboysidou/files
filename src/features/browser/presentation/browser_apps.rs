use gpui::{Context, NoAction, Pixels, Point, SharedString, Window};
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::features::applications::domain::entities::desktop_app::DesktopApp;
use crate::features::applications::domain::entities::launch_error::LaunchError;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::menu_state::{AppChoice, MenuState};
use crate::global_components::menu::menu_entry::MenuEntry;

impl BrowserView {
    pub(super) fn open_with(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let paths = self.active_pane().read(cx).target_paths();
        let Some(first) = paths.first().cloned() else {
            return;
        };
        let position = window.mouse_position();
        let catalog = self.setup.apps.clone();
        self.open_with_task = Some(cx.spawn_in(window, async move |this, cx| {
            let apps = cx
                .background_executor()
                .spawn(async move { catalog.apps_for(&first) })
                .await;
            let _ = this.update_in(cx, |this, window, cx| {
                let menu = open_with_menu(AppChoice { apps, paths }, position);
                this.present_open_with(menu, window, cx)
            });
        }));
    }

    fn present_open_with(
        &mut self,
        menu: Option<MenuState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(menu) = menu else {
            return self.show_toast(t!("apps.none").to_string(), false, cx);
        };
        self.menu = Some(menu);
        window.focus(&self.menu_focus, cx);
        cx.notify();
    }

    pub(super) fn launch_choice(
        &mut self,
        choice: AppChoice,
        index: usize,
        cx: &mut Context<Self>,
    ) {
        let Some(app) = choice.apps.get(index) else {
            return;
        };
        let outcome = self.setup.app_launcher.launch(app, &choice.paths);
        self.report_launch(outcome, cx);
    }

    pub(super) fn open_terminal(&mut self, cx: &mut Context<Self>) {
        let pane = self.active_pane().read(cx);
        let Some(directory) = pane.directory().or_else(|| self.setup.home.clone()) else {
            return;
        };
        let outcome = self.setup.app_launcher.open_terminal(&directory);
        self.report_launch(outcome, cx);
    }

    fn report_launch(&mut self, outcome: Result<(), LaunchError>, cx: &mut Context<Self>) {
        let message = match outcome {
            Ok(()) => return,
            Err(LaunchError::NoTerminal) => t!("apps.no_terminal").to_string(),
            Err(LaunchError::BadCommand) => t!("apps.bad_command").to_string(),
            Err(LaunchError::Spawn(reason)) => t!("apps.failed", reason = reason).to_string(),
        };
        self.show_toast(message, false, cx);
    }
}

fn open_with_menu(choice: AppChoice, position: Point<Pixels>) -> Option<MenuState> {
    if choice.apps.is_empty() {
        return None;
    }
    Some(MenuState {
        position,
        entries: choice.apps.iter().map(app_entry).collect(),
        hot: 0,
        apps: Some(choice),
    })
}

fn app_entry(app: &DesktopApp) -> MenuEntry {
    MenuEntry {
        label: SharedString::from(app.name.clone()),
        icon: if app.terminal {
            Icon::Terminal
        } else {
            Icon::Open
        },
        shortcut: SharedString::default(),
        action: Box::new(NoAction),
        enabled: true,
        danger: false,
        separated: false,
    }
}
