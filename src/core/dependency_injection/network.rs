use std::sync::Arc;

use crate::features::network::data::repositories::gio_network_mounter::GioNetworkMounter;
use crate::features::network::domain::repositories::network_mounter::NetworkMounter;

pub fn network_mounter() -> Arc<dyn NetworkMounter> {
    Arc::new(GioNetworkMounter)
}
