use std::fs::File;
use std::io;
use std::path::Path;

use rustix::fs::syncfs;

pub fn sync_folder(folder: &Path) -> io::Result<()> {
    syncfs(File::open(folder)?)?;
    Ok(())
}
