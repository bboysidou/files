use std::sync::Arc;

use crate::features::archives::data::repositories::bsdtar_extractor::BsdtarExtractor;
use crate::features::archives::domain::repositories::archive_extractor::ArchiveExtractor;

pub fn archive_extractor() -> Arc<dyn ArchiveExtractor> {
    Arc::new(BsdtarExtractor)
}
