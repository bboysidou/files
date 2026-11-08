use std::sync::Arc;

use crate::features::archives::data::repositories::bsdtar_archive_tool::BsdtarArchiveTool;
use crate::features::archives::domain::repositories::archive_tool::ArchiveTool;

pub fn archive_tool() -> Arc<dyn ArchiveTool> {
    Arc::new(BsdtarArchiveTool)
}
