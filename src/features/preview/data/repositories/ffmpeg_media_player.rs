use std::io::Read;
use std::process::{Child, ChildStdout};

use crate::core::constants::preview::BGRA_BYTES_PER_PIXEL;
use crate::features::preview::data::datasources::media_stream::{
    spawn_audio_stream, spawn_video_stream,
};
use crate::features::preview::domain::entities::playback_request::PlaybackRequest;
use crate::features::preview::domain::entities::video_frame::VideoFrame;
use crate::features::preview::domain::repositories::media_player::{MediaPlayer, Playback};

pub struct FfmpegMediaPlayer;

impl MediaPlayer for FfmpegMediaPlayer {
    fn start(&self, request: &PlaybackRequest) -> Option<Box<dyn Playback>> {
        let (width, height) = image::image_dimensions(&request.poster)
            .ok()
            .filter(|(width, height)| *width > 0 && *height > 0)?;
        let mut video = spawn_video_stream(request, (width, height))?;
        let Some(stdout) = video.stdout.take() else {
            stop(&mut video);
            return None;
        };
        let audio = request.audio.then(|| spawn_audio_stream(request)).flatten();
        Some(Box::new(FfmpegPlayback {
            video,
            audio,
            stdout,
            width,
            height,
        }))
    }
}

struct FfmpegPlayback {
    video: Child,
    audio: Option<Child>,
    stdout: ChildStdout,
    width: u32,
    height: u32,
}

impl Playback for FfmpegPlayback {
    fn next_frame(&mut self) -> Option<VideoFrame> {
        let length = self.width as usize * self.height as usize * BGRA_BYTES_PER_PIXEL;
        let mut pixels = vec![0; length];
        self.stdout.read_exact(&mut pixels).ok()?;
        Some(VideoFrame {
            pixels,
            width: self.width,
            height: self.height,
        })
    }
}

impl Drop for FfmpegPlayback {
    fn drop(&mut self) {
        stop(&mut self.video);
        if let Some(audio) = self.audio.as_mut() {
            stop(audio);
        }
    }
}

fn stop(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}
