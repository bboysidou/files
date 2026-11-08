use std::path::Path;
use std::time::SystemTime;

use crate::features::thumbnails::domain::entities::thumbnail::Thumbnail;

pub trait ThumbnailStore: Send + Sync {
    fn thumbnail(&self, path: &Path, modified: SystemTime) -> Thumbnail;
}
