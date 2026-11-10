use std::path::PathBuf;

use gpui::{
    App, Div, IntoElement, MouseButton, ObjectFit, RenderOnce, Window, canvas, div, img,
    prelude::*, px, relative,
};

use gpui::Stateful;
use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::metrics::QUICK_LOOK_STAGE_PAD;
use crate::features::browser::presentation::state::pan_gestures::PanGestures;
use crate::global_components::empty_state::EmptyState;

#[derive(IntoElement)]
pub struct PreviewImage {
    pub path: PathBuf,
    pub zoom: f32,
    pub pan: (f32, f32),
    pub gestures: Option<PanGestures>,
}

impl RenderOnce for PreviewImage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (pan_x, pan_y) = self.pan;
        let image = img(self.path)
            .object_fit(ObjectFit::Contain)
            .flex_shrink_0()
            .relative()
            .left(relative(-pan_x))
            .top(relative(-pan_y))
            .w(relative(self.zoom))
            .h(relative(self.zoom))
            .with_fallback(|| {
                EmptyState {
                    icon: Icon::Image,
                    title: t!("ql.failed.title").to_string().into(),
                    body: t!("ql.failed.body").to_string().into(),
                }
                .into_any_element()
            });
        let stage = div()
            .id("preview-image-stage")
            .relative()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .p(px(QUICK_LOOK_STAGE_PAD))
            .child(image);
        match self.gestures {
            Some(gestures) => with_gestures(stage, gestures),
            None => stage,
        }
    }
}

fn with_gestures(stage: Stateful<Div>, gestures: PanGestures) -> Stateful<Div> {
    let PanGestures {
        stage: size,
        on_press,
        on_drag,
        on_release,
    } = gestures;
    stage
        .child(
            canvas(move |bounds, _, _| size.set(bounds.size), |_, _, _, _| {})
                .absolute()
                .size_full(),
        )
        .cursor_grab()
        .on_mouse_down(MouseButton::Left, move |event, window, cx| {
            on_press(event.position, window, cx)
        })
        .on_mouse_move(move |event, window, cx| {
            if event.pressed_button == Some(MouseButton::Left) {
                on_drag(event.position, window, cx);
            }
        })
        .on_mouse_up(MouseButton::Left, move |_, window, cx| {
            on_release(window, cx)
        })
}
