use std::path::PathBuf;
use std::sync::Arc;

use gpui::{
    App, ImageSource, IntoElement, ObjectFit, RenderImage, RenderOnce, Window, div, img,
    prelude::*, px, rgba,
};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{FG, SCRIM};
use crate::core::theme::metrics::{PLAY_BUTTON_SIZE, PLAY_ICON_NUDGE, PLAY_ICON_SIZE, TOOLBAR_GAP};
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_glyph::IconGlyph;
use crate::global_components::progress_bar::ProgressBar;

#[derive(IntoElement)]
pub struct VideoStage {
    pub poster: PathBuf,
    pub live: Option<Arc<RenderImage>>,
    pub playing: bool,
    pub progress: Option<f32>,
    pub on_toggle: ClickHandler,
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
        let play = div()
            .absolute()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .size(px(PLAY_BUTTON_SIZE))
                    .rounded_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .pl(px(PLAY_ICON_NUDGE))
                    .bg(rgba(SCRIM))
                    .child(IconGlyph::new(Icon::Play, FG).size(PLAY_ICON_SIZE)),
            );
        let on_toggle = self.on_toggle;
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
                    .when(!self.playing, |stage| stage.child(play)),
            )
            .when_some(self.progress, |stage, fraction| {
                stage.child(div().flex().child(ProgressBar {
                    fraction: Some(fraction),
                }))
            })
    }
}
