use crate::features::network::domain::entities::ssh_host::SshHost;

pub trait SshHostSource: Send + Sync {
    fn hosts(&self) -> Vec<SshHost>;
}
