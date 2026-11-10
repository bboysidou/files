use crate::features::network::data::datasources::ssh_config_hosts::ssh_config_hosts;
use crate::features::network::domain::entities::ssh_host::SshHost;
use crate::features::network::domain::repositories::ssh_host_source::SshHostSource;

pub struct SshConfigHostSource;

impl SshHostSource for SshConfigHostSource {
    fn hosts(&self) -> Vec<SshHost> {
        dirs::home_dir().map_or_else(Vec::new, |home| ssh_config_hosts(&home))
    }
}
