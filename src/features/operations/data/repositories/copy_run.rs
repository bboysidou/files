use std::path::Path;

use crate::core::helpers::files::filesystem_kind_of::filesystem_kind_of;
use crate::features::operations::data::repositories::writeback::Writeback;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub struct CopyRun<'a> {
    pub sink: &'a dyn ProgressSink,
    pub writeback: Writeback,
    pub safe_names: bool,
}

impl<'a> CopyRun<'a> {
    pub fn for_destination(destination: &Path, sink: &'a dyn ProgressSink) -> Self {
        Self {
            sink,
            writeback: Writeback::for_destination(destination),
            safe_names: filesystem_kind_of(destination).restricts_names(),
        }
    }
}
