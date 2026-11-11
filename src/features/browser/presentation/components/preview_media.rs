use std::path::PathBuf;

use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::helpers::format::duration_text::duration_text;
use crate::core::theme::colors::{FG, FG_MUTED};
use crate::core::theme::metrics::{FOLDER_PREVIEW_GAP, FOLDER_PREVIEW_PAD_X, TOOLBAR_GAP};
use crate::features::browser::presentation::components::preview_image::PreviewImage;
use crate::features::browser::presentation::components::video_stage::VideoStage;
use crate::features::preview::domain::entities::media_facts::MediaFacts;

const KILO: f64 = 1000.0;

#[derive(IntoElement)]
pub struct PreviewMedia {
    pub frame: Option<PathBuf>,
    pub facts: MediaFacts,
    pub zoom: f32,
    pub video: Option<VideoStage>,
}

impl PreviewMedia {
    fn rows(facts: &MediaFacts) -> Vec<(String, String)> {
        let mut rows = Vec::new();
        if let Some(duration) = facts.duration_ms {
            rows.push((t!("media.duration").to_string(), duration_text(duration)));
        }
        if let Some(video) = &facts.video {
            let value = format!("{} · {}×{}", video.codec, video.width, video.height);
            rows.push((t!("media.video").to_string(), value));
        }
        if let Some(audio) = &facts.audio {
            let rate = format!("{:.1}", audio.sample_rate as f64 / KILO)
                .trim_end_matches(".0")
                .to_owned();
            let value = t!(
                "media.audio_value",
                codec = audio.codec,
                rate = rate,
                channels = audio.channels
            );
            rows.push((t!("media.audio").to_string(), value.to_string()));
        }
        rows
    }
}

impl RenderOnce for PreviewMedia {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let rows = Self::rows(&self.facts).into_iter().map(|(label, value)| {
            div()
                .flex()
                .gap(px(FOLDER_PREVIEW_GAP))
                .px(px(FOLDER_PREVIEW_PAD_X))
                .child(div().text_color(rgb(FG_MUTED)).child(label))
                .child(div().text_color(rgb(FG)).child(value))
        });
        let has_video = self.video.is_some();
        div()
            .flex()
            .flex_col()
            .size_full()
            .gap(px(TOOLBAR_GAP))
            .when_some(self.video, |stage, video| {
                stage.child(div().flex_1().min_h_0().child(video))
            })
            .when_some(self.frame.filter(|_| !has_video), |stage, frame| {
                stage.child(div().flex_1().min_h_0().child(PreviewImage {
                    path: frame,
                    zoom: self.zoom,
                    pan: (0.0, 0.0),
                    gestures: None,
                }))
            })
            .children(rows)
    }
}
