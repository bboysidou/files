use std::sync::Arc;

use crate::features::browser::domain::repositories::directory_repository::DirectoryRepository;
use crate::features::preview::data::repositories::local_preview_source::LocalPreviewSource;
use crate::features::preview::domain::repositories::preview_source::PreviewSource;

pub fn preview_source(directories: Arc<dyn DirectoryRepository>) -> Arc<dyn PreviewSource> {
    Arc::new(LocalPreviewSource::new(directories))
}
