use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use gpui::{
    App, Context, Entity, FocusHandle, Focusable, IntoElement, Render, Subscription, Task, Window,
    div, prelude::*, px, rgb,
};

use crate::core::constants::app::FONT_FAMILY;
use crate::core::constants::paths::PATH_SEPARATOR;
use crate::core::dependency_injection::browser::directory_watcher;
use crate::core::dependency_injection::browser_setup::browser_setup;
use crate::core::dependency_injection::devices::device_repository;
use crate::core::dependency_injection::network::mount_watcher;
use crate::core::dependency_injection::network::{network_mounter, ssh_host_source};
use crate::core::dependency_injection::phones::{phone_repository, volume_watcher};
use crate::core::dependency_injection::settings::settings_repository;
use crate::core::enums::places::place_kind::PlaceKind;
use crate::core::enums::tags::tag_color::TagColor;
use crate::core::theme::colors::{FG, FRAME};
use crate::core::theme::metrics::{FRAME_GAP, TEXT_BASE};
use crate::features::browser::domain::repositories::directory_watcher::DirectoryWatcher;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::drop_request::DropRequest;
use crate::features::devices::domain::entities::device::Device;
use crate::features::devices::domain::repositories::device_repository::DeviceRepository;
use crate::features::places::domain::entities::place::{Place, PlaceTarget};
use crate::features::places::domain::repositories::places_repository::PlacesRepository;
use crate::features::settings::domain::entities::settings::Settings;
use crate::features::settings::domain::repositories::settings_repository::SettingsRepository;
use crate::features::shell::presentation::actions::ToggleRail;
use crate::features::shell::presentation::state::network_rail::NetworkRail;
use crate::features::shell::presentation::state::open_shells::{OpenShell, OpenShells};
use crate::features::shell::presentation::state::phone_rail::PhoneRail;
use crate::features::shell::presentation::state::rail_keys::RailKeys;
use crate::features::shell::presentation::state::rail_motion::RailMotion;
use crate::features::shell::presentation::state::rail_stop::RailStop;
use crate::features::shell::presentation::state::shell_start::{ShellStart, WindowRole};
use crate::global_components::handlers::{ClickHandler, IndexHandler, IndexPathsHandler};

pub struct ShellView {
    pub(super) places: Vec<Place>,
    pub(super) browser: Entity<BrowserView>,
    pub(super) rail: RailMotion,
    pub(super) rail_keys: RailKeys,
    pub(super) settings_repository: Arc<dyn SettingsRepository>,
    pub(super) saved: Settings,
    pub(super) save_task: Option<Task<()>>,
    pub(super) home_target: PlaceTarget,
    pub(super) devices: Vec<Device>,
    pub(super) device_repository: Arc<dyn DeviceRepository>,
    pub(super) directory_watcher: Arc<dyn DirectoryWatcher>,
    pub(super) device_task: Option<Task<()>>,
    pub(super) device_list_task: Option<Task<()>>,
    pub(super) device_watch: Option<Task<()>>,
    pub(super) network: NetworkRail,
    pub(super) phones: PhoneRail,
    pub(super) role: WindowRole,
    _browser_changes: Subscription,
    _quit: Subscription,
}

impl ShellView {
    pub fn new(
        repository: &dyn PlacesRepository,
        start: ShellStart,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let places = repository.places();
        let home = repository.home();
        let home_target = Self::start_target(&places, home.as_ref());
        let settings_repository = settings_repository();
        let saved = settings_repository.load();
        let restore = Self::starting_snapshot(&saved, start.role, start.tabs.clone());
        let setup = browser_setup(places.clone(), home, home_target.clone());
        let browser = cx.new(|cx| BrowserView::new(setup, restore, window, cx));
        let browser_changes = cx.observe(&browser, |this, _, cx| {
            this.persist(cx);
            this.note_location(cx);
            this.sync_key_preset(cx);
            cx.notify();
        });
        let quit = cx.on_app_quit(|this, cx| {
            this.flush(cx);
            async {}
        });
        cx.on_release(|_, cx| forget_shell(cx)).detach();
        let mut shell = Self {
            places,
            browser,
            rail: RailMotion::starting(saved.sidebar_open),
            rail_keys: RailKeys {
                focus: cx.focus_handle(),
                cursor: RailStop::Place(0),
            },
            settings_repository,
            saved,
            save_task: None,
            home_target,
            devices: Vec::new(),
            device_repository: device_repository(),
            directory_watcher: directory_watcher(),
            device_task: None,
            device_list_task: None,
            device_watch: None,
            network: NetworkRail::new(network_mounter(), mount_watcher(), ssh_host_source()),
            phones: PhoneRail::new(phone_repository(), volume_watcher()),
            role: start.role,
            _browser_changes: browser_changes,
            _quit: quit,
        };
        shell.start((start.launch, start.tabs.is_some()), window, cx);
        shell
    }

    fn start_target(places: &[Place], home: Option<&PathBuf>) -> PlaceTarget {
        places
            .iter()
            .find(|place| place.kind == PlaceKind::Home)
            .map(|place| place.target.clone())
            .unwrap_or_else(|| {
                PlaceTarget::Directory(
                    home.cloned()
                        .unwrap_or_else(|| PathBuf::from(PATH_SEPARATOR)),
                )
            })
    }

    fn toggle_rail(&mut self, cx: &mut Context<Self>) {
        self.rail = self.rail.toggled();
        self.persist(cx);
        cx.notify();
    }

    pub(super) fn select_place(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(target) = self.places.get(index).map(|place| place.target.clone()) else {
            return;
        };
        self.browser
            .update(cx, |browser, cx| browser.navigate(target, cx));
    }

    pub(super) fn place_drop_handler(&self) -> IndexPathsHandler {
        let targets: Vec<PlaceTarget> = self
            .places
            .iter()
            .map(|place| place.target.clone())
            .collect();
        let browser = self.browser.downgrade();
        Rc::new(move |index, sources, window, cx| {
            let Some(PlaceTarget::Directory(destination)) = targets.get(index).cloned() else {
                return;
            };
            let request = DropRequest {
                sources,
                destination,
                copy: window.modifiers().control,
            };
            let _ = browser.update(cx, |browser, cx| browser.drop_files(request, cx));
        })
    }

    pub(super) fn handlers(cx: &mut Context<Self>) -> (ClickHandler, IndexHandler) {
        let view = cx.entity().downgrade();
        let toggle_view = view.clone();
        let on_toggle: ClickHandler = Rc::new(move |_, cx| {
            let _ = toggle_view.update(cx, |this, cx| this.toggle_rail(cx));
        });
        let on_select: IndexHandler = Rc::new(move |index, _, cx| {
            let _ = view.update(cx, |this, cx| this.select_place(index, cx));
        });
        (on_toggle, on_select)
    }
}

impl ShellView {
    pub(super) fn browser_handlers(&self) -> (ClickHandler, ClickHandler, IndexHandler) {
        let job_browser = self.browser.downgrade();
        let on_cancel_job: ClickHandler = Rc::new(move |_, cx| {
            let _ = job_browser.update(cx, |browser, _| browser.cancel_job());
        });
        let search_browser = self.browser.downgrade();
        let on_search: ClickHandler = Rc::new(move |window, cx| {
            let _ = search_browser.update(cx, |browser, cx| browser.start_search(window, cx));
        });
        let browser = self.browser.downgrade();
        let on_tag: IndexHandler = Rc::new(move |index, window, cx| {
            let Some(tag) = TagColor::ALL.get(index).copied() else {
                return;
            };
            let _ = browser.update(cx, |browser, cx| browser.search_tag(tag, window, cx));
        });
        (on_cancel_job, on_search, on_tag)
    }
}

impl Render for ShellView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rail = self.rail_view(window, cx);
        Self::with_window_actions(Self::with_rail_actions(div(), cx), cx)
            .on_action(cx.listener(|this, _: &ToggleRail, _, cx| this.toggle_rail(cx)))
            .size_full()
            .flex()
            .gap(px(FRAME_GAP))
            .p(px(FRAME_GAP))
            .bg(rgb(FRAME))
            .font_family(FONT_FAMILY)
            .text_size(px(TEXT_BASE))
            .text_color(rgb(FG))
            .child(rail)
            .child(self.browser.clone())
    }
}

impl Focusable for ShellView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.browser.read(cx).focus_handle(cx)
    }
}

fn forget_shell(cx: &mut App) {
    if let Some(shells) = cx.try_global::<OpenShells>() {
        let live: Vec<OpenShell> = shells
            .0
            .iter()
            .filter(|shell| shell.view.upgrade().is_some())
            .map(|shell| OpenShell {
                view: shell.view.clone(),
                window: shell.window,
            })
            .collect();
        cx.set_global(OpenShells(live));
    }
}
