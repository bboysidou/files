use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::core::constants::network::{
    GIO_COMMAND, GIO_INFO, GIO_MOUNT, LOCAL_PATH_PREFIX, MOUNT_ALREADY,
};
use crate::features::network::domain::repositories::network_mounter::NetworkMounter;

pub struct GioNetworkMounter;

impl NetworkMounter for GioNetworkMounter {
    fn mount(&self, uri: &str) -> Result<PathBuf, String> {
        let mounted = Command::new(GIO_COMMAND)
            .args([GIO_MOUNT, uri])
            .stdin(Stdio::null())
            .output()
            .map_err(|error| error.to_string())?;
        let complaint = String::from_utf8_lossy(&mounted.stderr).trim().to_owned();
        if !mounted.status.success() && !complaint.contains(MOUNT_ALREADY) {
            return Err(complaint);
        }
        let info = Command::new(GIO_COMMAND)
            .args([GIO_INFO, uri])
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .map_err(|error| error.to_string())?;
        String::from_utf8_lossy(&info.stdout)
            .lines()
            .find_map(|line| line.strip_prefix(LOCAL_PATH_PREFIX))
            .map(|path| PathBuf::from(path.trim()))
            .ok_or(complaint)
    }
}
