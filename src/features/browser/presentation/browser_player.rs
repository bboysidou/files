use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use futures::StreamExt;
use futures::channel::mpsc::{self, Sender};
use gpui::{Context, RenderImage};
use image::{Frame, RgbaImage};

use crate::core::constants::preview::PLAYBACK_FRAME_QUEUE;
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::components::video_stage::VideoStage;
use crate::features::browser::presentation::state::player_state::PlayerState;
use crate::features::preview::domain::entities::media_facts::MediaFacts;
use crate::features::preview::domain::entities::playback_request::PlaybackRequest;
use crate::features::preview::domain::entities::preview_content::PreviewContent;
use crate::features::preview::domain::entities::video_frame::VideoFrame;
use crate::features::preview::domain::repositories::media_player::MediaPlayer;
use crate::global_components::handlers::ClickHandler;

impl BrowserView {
    pub(super) fn toggle_playback(&mut self, cx: &mut Context<Self>) {
        if self.quick_look.player.playing() {
            self.pause_playback(cx);
        } else {
            self.start_playback(cx);
        }
    }

    pub(super) fn stop_playback(&mut self, cx: &mut Context<Self>) {
        let player = std::mem::take(&mut self.quick_look.player);
        for image in [player.frame, player.retired].into_iter().flatten() {
            cx.drop_image(image, None);
        }
    }

    pub(super) fn video_stage(
        &self,
        poster: &Path,
        facts: &MediaFacts,
        cx: &mut Context<Self>,
    ) -> Option<VideoStage> {
        facts.video.as_ref()?;
        let player = &self.quick_look.player;
        let view = cx.entity().downgrade();
        let on_toggle: ClickHandler = Rc::new(move |_, cx| {
            let _ = view.update(cx, |this, cx| this.toggle_playback(cx));
        });
        let progress = facts
            .duration_ms
            .filter(|duration| *duration > 0 && (player.playing() || player.offset_ms > 0))
            .map(|duration| player.position_ms() as f32 / duration as f32);
        Some(VideoStage {
            poster: poster.to_path_buf(),
            live: player.frame.clone(),
            playing: player.playing(),
            progress,
            on_toggle,
        })
    }

    fn start_playback(&mut self, cx: &mut Context<Self>) {
        let Some(request) = self.playback_request() else {
            return;
        };
        let (sender, mut receiver) = mpsc::channel(PLAYBACK_FRAME_QUEUE);
        spawn_frame_reader(self.setup.player.clone(), request.clone(), sender);
        let task = cx.spawn(async move |this, cx| {
            while let Some(image) = receiver.next().await {
                if this
                    .update(cx, |this, cx| this.show_video_frame(image, cx))
                    .is_err()
                {
                    return;
                }
            }
            let _ = this.update(cx, |this, cx| {
                this.stop_playback(cx);
                cx.notify();
            });
        });
        let player = &mut self.quick_look.player;
        player.offset_ms = request.start_ms;
        player.started = Some(Instant::now());
        player.task = Some(task);
        cx.notify();
    }

    fn pause_playback(&mut self, cx: &mut Context<Self>) {
        let player = &mut self.quick_look.player;
        player.offset_ms = player.position_ms();
        player.started = None;
        player.task = None;
        cx.notify();
    }

    fn show_video_frame(&mut self, image: Arc<RenderImage>, cx: &mut Context<Self>) {
        let player: &mut PlayerState = &mut self.quick_look.player;
        let previous = player.frame.replace(image);
        if let Some(stale) = std::mem::replace(&mut player.retired, previous) {
            cx.drop_image(stale, None);
        }
        cx.notify();
    }

    fn playback_request(&self) -> Option<PlaybackRequest> {
        let state = self.quick_look.state.as_ref()?;
        let Some(PreviewContent::Media {
            frame: Some(poster),
            facts,
        }) = &state.content
        else {
            return None;
        };
        facts.video.as_ref()?;
        let offset = self.quick_look.player.offset_ms;
        let finished = facts.duration_ms.is_some_and(|duration| offset >= duration);
        Some(PlaybackRequest {
            path: state.entry.path.clone(),
            start_ms: if finished { 0 } else { offset },
            poster: poster.clone(),
            audio: facts.audio.is_some(),
        })
    }
}

fn spawn_frame_reader(
    player: Arc<dyn MediaPlayer>,
    request: PlaybackRequest,
    mut sender: Sender<Arc<RenderImage>>,
) {
    thread::spawn(move || {
        let Some(mut playback) = player.start(&request) else {
            return;
        };
        while let Some(frame) = playback.next_frame() {
            let Some(image) = render_image(frame) else {
                return;
            };
            if let Err(error) = sender.try_send(image)
                && error.is_disconnected()
            {
                return;
            }
        }
    });
}

fn render_image(frame: VideoFrame) -> Option<Arc<RenderImage>> {
    let buffer = RgbaImage::from_raw(frame.width, frame.height, frame.pixels)?;
    Some(Arc::new(RenderImage::new(vec![Frame::new(buffer)])))
}
