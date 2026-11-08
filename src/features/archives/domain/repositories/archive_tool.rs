use std::path::{Path, PathBuf};

use crate::features::archives::domain::entities::archive_error::ArchiveError;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub trait ArchiveTool: Send + Sync {
    fn extract(
        &self,
        archive: &Path,
        destination: &Path,
        sink: &dyn ProgressSink,
    ) -> Result<(), ArchiveError>;
    fn compress(
        &self,
        sources: &[PathBuf],
        archive: &Path,
        sink: &dyn ProgressSink,
    ) -> Result<(), ArchiveError>;
}
