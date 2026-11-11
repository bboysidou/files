use std::sync::Arc;

use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::preview::data::repositories::ffmpeg_media_player::FfmpegMediaPlayer;
use crate::features::preview::data::repositories::local_preview_source::LocalPreviewSource;
use crate::features::preview::domain::repositories::media_player::MediaPlayer;
use crate::features::preview::domain::repositories::preview_source::PreviewSource;

pub fn preview_source(directories: Arc<dyn DirectoryRepository>) -> Arc<dyn PreviewSource> {
    Arc::new(LocalPreviewSource::new(directories))
}

pub fn media_player() -> Arc<dyn MediaPlayer> {
    Arc::new(FfmpegMediaPlayer)
}
