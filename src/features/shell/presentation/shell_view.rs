use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use gpui::{
    App, Context, Entity, FocusHandle, Focusable, IntoElement, Render, Subscription, Task, Window,
    div, prelude::*, px, rgb,
};

use crate::core::constants::app::FONT_FAMILY;
use crate::core::constants::paths::PATH_SEPARATOR;
use crate::core::dependency_injection::browser::{
    directory_repository, directory_watcher, file_launcher,
};
use crate::core::dependency_injection::operations::{file_operations, system_clipboard};
use crate::core::dependency_injection::preview::preview_source;
use crate::core::dependency_injection::settings::settings_repository;
use crate::core::dependency_injection::thumbnails::thumbnail_store;
use crate::core::enums::places::place_kind::PlaceKind;
use crate::core::theme::colors::{FG, FRAME};
use crate::core::theme::metrics::{FRAME_GAP, TEXT_BASE};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::state::browser_setup::BrowserSetup;
use crate::features::places::domain::entities::place::{Place, PlaceTarget};
use crate::features::places::domain::repositories::places_repository::PlacesRepository;
use crate::features::settings::domain::entities::settings::Settings;
use crate::features::settings::domain::repositories::settings_repository::SettingsRepository;
use crate::features::shell::presentation::actions::ToggleRail;
use crate::features::shell::presentation::components::rail::Rail;
use crate::features::shell::presentation::state::rail_motion::RailMotion;
use crate::global_components::handlers::{ClickHandler, IndexHandler};

pub struct ShellView {
    places: Vec<Place>,
    pub(super) browser: Entity<BrowserView>,
    pub(super) rail: RailMotion,
    pub(super) settings_repository: Arc<dyn SettingsRepository>,
    pub(super) saved: Settings,
    pub(super) save_task: Option<Task<()>>,
    _browser_changes: Subscription,
    _quit: Subscription,
}

impl ShellView {
    pub fn new(
        repository: &dyn PlacesRepository,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let places = repository.places();
        let home = repository.home();
        let home_target = Self::start_target(&places, home.as_ref());
        let settings_repository = settings_repository();
        let saved = settings_repository.load();
        let restore = Self::restored_snapshot(&saved);
        let repository = directory_repository();
        let setup = BrowserSetup {
            preview: preview_source(repository.clone()),
            thumbnails: thumbnail_store(),
            repository,
            launcher: file_launcher(),
            watcher: directory_watcher(),
            operations: file_operations(),
            system_clipboard: system_clipboard(),
            places: places.clone(),
            home,
            home_target,
        };
        let browser = cx.new(|cx| BrowserView::new(setup, restore, window, cx));
        let browser_changes = cx.observe(&browser, |this, _, cx| {
            this.persist(cx);
            cx.notify();
        });
        let quit = cx.on_app_quit(|this, cx| {
            this.flush(cx);
            async {}
        });
        Self {
            places,
            browser,
            rail: RailMotion::starting(saved.sidebar_open),
            settings_repository,
            saved,
            save_task: None,
            _browser_changes: browser_changes,
            _quit: quit,
        }
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

    fn select_place(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(target) = self.places.get(index).map(|place| place.target.clone()) else {
            return;
        };
        self.browser
            .update(cx, |browser, cx| browser.navigate(target, cx));
    }

    fn handlers(cx: &mut Context<Self>) -> (ClickHandler, IndexHandler) {
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

impl Render for ShellView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (on_toggle, on_select) = Self::handlers(cx);
        let location = self.browser.read(cx).location(cx);
        let active = self
            .places
            .iter()
            .position(|place| place.target == location);
        div()
            .on_action(cx.listener(|this, _: &ToggleRail, _, cx| this.toggle_rail(cx)))
            .size_full()
            .flex()
            .gap(px(FRAME_GAP))
            .p(px(FRAME_GAP))
            .bg(rgb(FRAME))
            .font_family(FONT_FAMILY)
            .text_size(px(TEXT_BASE))
            .text_color(rgb(FG))
            .child(Rail {
                places: self.places.clone(),
                active,
                motion: self.rail,
                on_toggle,
                on_select,
            })
            .child(self.browser.clone())
    }
}

impl Focusable for ShellView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.browser.read(cx).focus_handle(cx)
    }
}
