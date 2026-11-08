use std::sync::Arc;

use crate::features::thumbnails::data::repositories::freedesktop_thumbnail_store::FreedesktopThumbnailStore;
use crate::features::thumbnails::domain::repositories::thumbnail_store::ThumbnailStore;

pub fn thumbnail_store() -> Arc<dyn ThumbnailStore> {
    Arc::new(FreedesktopThumbnailStore::new())
}
