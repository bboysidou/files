use std::path::PathBuf;
use std::rc::Rc;

use gpui::{
    Context, Entity, FocusHandle, Focusable, IntoElement, Render, Subscription, Task, Window, div,
    prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::constants::app::FONT_FAMILY;
use crate::core::dependency_injection::browser_setup::browser_setup;
use crate::core::dependency_injection::places::places_repository;
use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::colors::{FG, FRAME, SHEET};
use crate::core::theme::metrics::{FRAME_GAP, SHEET_RADIUS, TEXT_BASE};
use crate::features::browser::domain::entities::sort_order::SortOrder;
use crate::features::browser::domain::entities::view_options::ViewOptions;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::browser::presentation::state::pane_start::PaneStart;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::picker::domain::entities::picker_request::{
    PickerMode, PickerRequest, PickerResult,
};
use crate::features::picker::domain::entities::picker_shortcut::PickerShortcut;
use crate::features::picker::domain::repositories::picker_portal::PickerReply;
use crate::features::picker::presentation::components::picker_footer::PickerFooter;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::text_field::text_field_view::{TextField, TextFieldEvent};

pub struct PickerView {
    pub(super) pane: Entity<PaneView>,
    pub(super) shortcuts: Vec<PickerShortcut>,
    pub(super) devices_task: Option<Task<()>>,
    request: PickerRequest,
    name_field: Entity<TextField>,
    reply: Option<PickerReply>,
    _subscriptions: Vec<Subscription>,
}

impl PickerView {
    fn build_pane(
        request: &PickerRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<PaneView> {
        let places = places_repository();
        let home = places.home();
        let start = request
            .start_folder
            .clone()
            .or_else(|| home.clone())
            .unwrap_or_default();
        let setup = Rc::new(browser_setup(
            places.places(),
            home,
            PlaceTarget::Directory(start.clone()),
        ));
        let options = ViewOptions {
            show_hidden: false,
            sort: SortOrder::by_name(),
            view: ViewMode::List,
            columns: Default::default(),
        };
        let pane_start = PaneStart {
            tab: TabSnapshot::fresh(PlaceTarget::Directory(start)),
            options,
        };
        let pane = cx.new(|cx| PaneView::new(setup, pane_start, window, cx));
        pane.update(cx, |pane, _| pane.set_picker(true));
        pane
    }
}

impl PickerView {
    pub fn new(
        request: PickerRequest,
        reply: PickerReply,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let pane = Self::build_pane(&request, window, cx);
        let name = request.suggested_name.clone().unwrap_or_default();
        let name_field = cx.new(|cx| TextField::new(t!("picker.name").to_string(), cx));
        name_field.update(cx, |field, cx| field.set_text(&name, 0..name.len(), cx));
        let subscriptions = vec![
            cx.observe(&pane, |_, _, cx| cx.notify()),
            cx.subscribe_in(
                &pane,
                window,
                |this, _, event: &PaneEvent, window, cx| match event {
                    PaneEvent::Picked(paths) => this.finish(paths.clone(), window, cx),
                    PaneEvent::PickCancelled => this.cancel(window, cx),
                    _ => {}
                },
            ),
            cx.subscribe_in(
                &name_field,
                window,
                |this, _, event, window, cx| match event {
                    TextFieldEvent::Submitted => this.accept(window, cx),
                    TextFieldEvent::Cancelled => this.cancel(window, cx),
                    TextFieldEvent::Changed => {}
                },
            ),
        ];
        let mut view = Self {
            pane,
            shortcuts: Self::initial_shortcuts(),
            devices_task: None,
            request,
            name_field,
            reply: Some(reply),
            _subscriptions: subscriptions,
        };
        view.load_devices(cx);
        view
    }

    fn chosen(&self, cx: &Context<Self>) -> Vec<PathBuf> {
        let pane = self.pane.read(cx);
        match self.request.mode {
            PickerMode::Save => {
                let name = self.name_field.read(cx).text().trim().to_string();
                pane.directory()
                    .filter(|_| !name.is_empty())
                    .map(|folder| folder.join(name))
                    .into_iter()
                    .collect()
            }
            PickerMode::Open if self.request.directory => pane.directory().into_iter().collect(),
            PickerMode::Open => pane
                .target_paths()
                .into_iter()
                .filter(|path| !path.is_dir())
                .collect(),
        }
    }

    fn accept(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let chosen = self.chosen(cx);
        self.finish(chosen, window, cx);
    }

    fn finish(&mut self, mut chosen: Vec<PathBuf>, window: &mut Window, cx: &mut Context<Self>) {
        if self.request.mode == PickerMode::Save || self.request.directory {
            chosen = self.chosen(cx);
        }
        if chosen.is_empty() {
            return;
        }
        if !self.request.multiple {
            chosen.truncate(1);
        }
        if let Some(reply) = self.reply.take() {
            let _ = reply.send(PickerResult::Chosen(chosen));
        }
        window.remove_window();
    }

    fn cancel(&mut self, window: &mut Window, _cx: &mut Context<Self>) {
        if let Some(reply) = self.reply.take() {
            let _ = reply.send(PickerResult::Cancelled);
        }
        window.remove_window();
    }

    fn handler(
        cx: &mut Context<Self>,
        action: fn(&mut Self, &mut Window, &mut Context<Self>),
    ) -> ClickHandler {
        let view = cx.entity().downgrade();
        Rc::new(move |window, cx| {
            let _ = view.update(cx, |this, cx| action(this, window, cx));
        })
    }

    fn accept_label(&self) -> String {
        self.request
            .accept_label
            .clone()
            .unwrap_or_else(|| match self.request.mode {
                PickerMode::Open => t!("picker.open").to_string(),
                PickerMode::Save => t!("picker.save").to_string(),
            })
    }
}

impl Render for PickerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar = self.sidebar(cx);
        let footer = PickerFooter {
            name_field: (self.request.mode == PickerMode::Save).then(|| self.name_field.clone()),
            cancel_label: t!("picker.cancel").to_string().into(),
            accept_label: self.accept_label().into(),
            on_cancel: Self::handler(cx, Self::cancel),
            on_accept: Self::handler(cx, Self::accept),
        };
        div()
            .size_full()
            .p(px(FRAME_GAP))
            .bg(rgb(FRAME))
            .font_family(FONT_FAMILY)
            .text_size(px(TEXT_BASE))
            .text_color(rgb(FG))
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .rounded(px(SHEET_RADIUS))
                    .bg(rgb(SHEET))
                    .child(
                        div().flex().flex_1().min_h_0().child(sidebar).child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .min_w_0()
                                .p(px(FRAME_GAP))
                                .child(self.pane.clone()),
                        ),
                    )
                    .child(footer),
            )
    }
}

impl Focusable for PickerView {
    fn focus_handle(&self, cx: &gpui::App) -> FocusHandle {
        if self.request.mode == PickerMode::Save {
            self.name_field.read(cx).focus_handle(cx)
        } else {
            self.pane.read(cx).focus_handle(cx)
        }
    }
}

impl Drop for PickerView {
    fn drop(&mut self) {
        if let Some(reply) = self.reply.take() {
            let _ = reply.send(PickerResult::Cancelled);
        }
    }
}
