use std::sync::Arc;

use crate::features::integration::data::repositories::dbus_instance_bridge::DbusInstanceBridge;
use crate::features::integration::data::repositories::xdg_default_handler::XdgDefaultHandler;
use crate::features::integration::domain::repositories::default_handler::DefaultHandler;
use crate::features::integration::domain::repositories::instance_bridge::InstanceBridge;

pub fn instance_bridge() -> Arc<dyn InstanceBridge> {
    Arc::new(DbusInstanceBridge)
}

pub fn default_handler() -> Arc<dyn DefaultHandler> {
    Arc::new(XdgDefaultHandler)
}
