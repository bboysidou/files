use std::fs;
use std::path::Path;

use crate::core::constants::sandbox::{PROC_DIR, PROC_FD_DIR, PROC_FDINFO_DIR, PROC_POSITION_KEY};

pub fn open_file_position(pids: &[u32], wanted: &dyn Fn(&Path) -> bool) -> u64 {
    pids.iter().map(|pid| process_position(*pid, wanted)).sum()
}

fn process_position(pid: u32, wanted: &dyn Fn(&Path) -> bool) -> u64 {
    let process = Path::new(PROC_DIR).join(pid.to_string());
    let Ok(fds) = fs::read_dir(process.join(PROC_FD_DIR)) else {
        return 0;
    };
    fds.flatten()
        .filter(|fd| fs::read_link(fd.path()).is_ok_and(|link| link.is_file() && wanted(&link)))
        .filter_map(|fd| {
            let info =
                fs::read_to_string(process.join(PROC_FDINFO_DIR).join(fd.file_name())).ok()?;
            info.lines()
                .find_map(|line| line.strip_prefix(PROC_POSITION_KEY))
                .and_then(|value| value.trim().parse::<u64>().ok())
        })
        .sum()
}
