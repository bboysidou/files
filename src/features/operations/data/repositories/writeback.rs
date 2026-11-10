use std::fs::File;
use std::io;
use std::path::Path;

use rustix::fs::syncfs;

use crate::core::constants::files::WRITEBACK_WINDOW_BYTES;
use crate::core::helpers::files::slow_destination::slow_destination;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub struct Writeback {
    folder: Option<File>,
    pending: u64,
}

impl Writeback {
    pub fn for_destination(destination: &Path) -> Self {
        let folder = destination
            .parent()
            .filter(|parent| slow_destination(parent))
            .and_then(|parent| File::open(parent).ok());
        Self { folder, pending: 0 }
    }

    pub fn wrote(&mut self, bytes: u64, sink: &dyn ProgressSink) -> io::Result<()> {
        if self.folder.is_none() {
            sink.advance(bytes);
            return Ok(());
        }
        self.pending += bytes;
        if self.pending >= WRITEBACK_WINDOW_BYTES {
            self.flush(sink)?;
        }
        Ok(())
    }

    pub fn flush(&mut self, sink: &dyn ProgressSink) -> io::Result<()> {
        let Some(folder) = &self.folder else {
            return Ok(());
        };
        syncfs(folder)?;
        sink.advance(self.pending);
        self.pending = 0;
        Ok(())
    }
}
