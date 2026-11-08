use std::path::PathBuf;
use std::rc::Rc;

use gpui::{
    App, Context, FocusHandle, Focusable, IntoElement, Render, Window, div, prelude::*, px, rgb,
};

use crate::core::constants::app::FONT_FAMILY;
use crate::core::enums::places::place_kind::PlaceKind;
use crate::core::theme::colors::{FG, FRAME};
use crate::core::theme::metrics::{FRAME_GAP, TEXT_BASE};
use crate::features::places::domain::entities::place::Place;
use crate::features::places::domain::repositories::places_repository::PlacesRepository;
use crate::features::shell::presentation::actions::ToggleRail;
use crate::features::shell::presentation::components::rail::Rail;
use crate::features::shell::presentation::components::sheet::Sheet;
use crate::features::shell::presentation::state::rail_motion::RailMotion;
use crate::global_components::handlers::{ClickHandler, IndexHandler};

pub struct ShellView {
    places: Vec<Place>,
    home: Option<PathBuf>,
    active: usize,
    rail: RailMotion,
    focus_handle: FocusHandle,
}

impl ShellView {
    pub fn new(repository: &dyn PlacesRepository, cx: &mut Context<Self>) -> Self {
        let places = repository.places();
        let active = places
            .iter()
            .position(|place| place.kind == PlaceKind::Home)
            .unwrap_or_default();
        Self {
            places,
            home: repository.home(),
            active,
            rail: RailMotion::opened(),
            focus_handle: cx.focus_handle(),
        }
    }

    fn toggle_rail(&mut self, cx: &mut Context<Self>) {
        self.rail = self.rail.toggled();
        cx.notify();
    }

    fn select_place(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.places.len() && index != self.active {
            self.active = index;
            cx.notify();
        }
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
        div()
            .track_focus(&self.focus_handle)
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
                active: self.active,
                motion: self.rail,
                on_toggle,
                on_select,
            })
            .child(Sheet {
                place: self.places[self.active].clone(),
                home: self.home.clone(),
            })
    }
}

impl Focusable for ShellView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
