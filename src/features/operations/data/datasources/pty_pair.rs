use std::fs::{File, OpenOptions};
use std::io;

use rustix::pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt};

pub fn pty_pair() -> io::Result<(File, File)> {
    let master = openpt(OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC)?;
    grantpt(&master)?;
    unlockpt(&master)?;
    let name = ptsname(&master, Vec::new())?;
    let follower = OpenOptions::new()
        .read(true)
        .write(true)
        .open(name.to_string_lossy().as_ref())?;
    Ok((File::from(master), follower))
}
