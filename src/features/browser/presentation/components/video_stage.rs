use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use gpui::{
    App, Bounds, ImageSource, IntoElement, MouseButton, ObjectFit, Pixels, RenderImage, RenderOnce,
    Window, canvas, div, img, prelude::*, px,
};

use crate::core::theme::metrics::{PLAY_BUTTON_SIZE, PLAY_ICON_SIZE, SEEK_HIT_PAD, TOOLBAR_GAP};
use crate::global_components::handlers::{AmountHandler, ClickHandler};
use crate::global_components::play_badge::PlayBadge;
use crate::global_components::progress_bar::ProgressBar;

#[derive(IntoElement)]
pub struct VideoStage {
    pub poster: PathBuf,
    pub live: Option<Arc<RenderImage>>,
    pub playing: bool,
    pub progress: Option<f32>,
    pub on_toggle: ClickHandler,
    pub on_seek: AmountHandler,
}

impl RenderOnce for VideoStage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let source: ImageSource = match self.live {
            Some(image) => image.into(),
            None => self.poster.into(),
        };
        let picture = img(source)
            .object_fit(ObjectFit::Contain)
            .absolute()
            .size_full();
        let on_toggle = self.on_toggle;
        let on_seek = self.on_seek;
        div()
            .flex()
            .flex_col()
            .size_full()
            .gap(px(TOOLBAR_GAP))
            .child(
                div()
                    .id("video-stage")
                    .relative()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .overflow_hidden()
                    .cursor_pointer()
                    .on_click(move |_, window, cx| on_toggle(window, cx))
                    .child(picture)
                    .when(!self.playing, |stage| {
                        stage.child(PlayBadge {
                            size: PLAY_BUTTON_SIZE,
                            icon_size: PLAY_ICON_SIZE,
                        })
                    }),
            )
            .when_some(self.progress, |stage, fraction| {
                stage.child(seek_bar(fraction, on_seek))
            })
    }
}

fn seek_bar(fraction: f32, on_seek: AmountHandler) -> impl IntoElement {
    let bounds: Rc<Cell<Bounds<Pixels>>> = Rc::default();
    let measured = bounds.clone();
    div()
        .id("video-seek")
        .relative()
        .flex()
        .py(px(SEEK_HIT_PAD))
        .cursor_pointer()
        .child(
            canvas(move |area, _, _| measured.set(area), |_, _, _, _| {})
                .absolute()
                .size_full(),
        )
        .child(ProgressBar {
            fraction: Some(fraction),
        })
        .on_mouse_down(MouseButton::Left, move |event, window, cx| {
            cx.stop_propagation();
            let area = bounds.get();
            if area.size.width > px(0.0) {
                let offset = (event.position.x - area.origin.x) / area.size.width;
                on_seek(offset.clamp(0.0, 1.0), window, cx);
            }
        })
}
