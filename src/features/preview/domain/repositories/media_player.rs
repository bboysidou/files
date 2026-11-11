use crate::features::preview::domain::entities::playback_request::PlaybackRequest;
use crate::features::preview::domain::entities::video_frame::VideoFrame;

pub trait Playback: Send {
    fn next_frame(&mut self) -> Option<VideoFrame>;
}

pub trait MediaPlayer: Send + Sync {
    fn start(&self, request: &PlaybackRequest) -> Option<Box<dyn Playback>>;
}
