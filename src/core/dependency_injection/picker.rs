use std::sync::Arc;

use crate::features::picker::data::repositories::dbus_picker_portal::DbusPickerPortal;
use crate::features::picker::data::repositories::xdg_portal_registration::XdgPortalRegistration;
use crate::features::picker::domain::repositories::picker_portal::PickerPortal;
use crate::features::picker::domain::repositories::portal_registration::PortalRegistration;

pub fn picker_portal() -> Arc<dyn PickerPortal> {
    Arc::new(DbusPickerPortal)
}

pub fn portal_registration() -> Arc<dyn PortalRegistration> {
    Arc::new(XdgPortalRegistration)
}
