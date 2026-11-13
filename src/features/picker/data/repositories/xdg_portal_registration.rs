use std::env;
use std::fs;
use std::path::Path;

use crate::core::constants::integration::{
    APP_BUS_NAME, APP_SERVICE_FILE, ARG_PORTAL, DBUS_SERVICES_DIR, PORTAL_BUS_NAME,
    PORTAL_FILE_NAME, PORTAL_SERVICE_FILE, SYSTEM_PORTALS_DIR,
};
use crate::core::constants::settings::SETTINGS_DIR;
use crate::features::picker::domain::repositories::portal_registration::PortalRegistration;

const PORTAL_INTERFACE: &str = "org.freedesktop.impl.portal.FileChooser";
const PORTAL_SHORT_NAME: &str = "files";

pub struct XdgPortalRegistration;

impl XdgPortalRegistration {
    fn write(path: &Path, contents: &str) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::write(path, contents).map_err(|error| error.to_string())
    }

    fn service_entry(name: &str) -> Result<String, String> {
        let executable = env::current_exe().map_err(|error| error.to_string())?;
        Ok(format!(
            "[D-BUS Service]\nName={name}\nExec={} {ARG_PORTAL}\n",
            executable.display()
        ))
    }

    fn portal_entry() -> String {
        format!("[portal]\nDBusName={PORTAL_BUS_NAME}\nInterfaces={PORTAL_INTERFACE};\n")
    }

    fn instructions(portal: &Path) -> String {
        format!(
            "D-Bus service written. Finish with:\n\n  sudo install -Dm644 {} {SYSTEM_PORTALS_DIR}/{PORTAL_FILE_NAME}\n\nthen add to ~/.config/xdg-desktop-portal/portals.conf (or <desktop>-portals.conf):\n\n  [preferred]\n  {PORTAL_INTERFACE}={PORTAL_SHORT_NAME}\n\nand restart the portal:\n\n  systemctl --user restart xdg-desktop-portal\n",
            portal.display()
        )
    }
}

impl PortalRegistration for XdgPortalRegistration {
    fn register(&self) -> Result<String, String> {
        let data = dirs::data_dir().ok_or("no data directory")?;
        let services = data.join(DBUS_SERVICES_DIR);
        Self::write(
            &services.join(PORTAL_SERVICE_FILE),
            &Self::service_entry(PORTAL_BUS_NAME)?,
        )?;
        Self::write(
            &services.join(APP_SERVICE_FILE),
            &Self::service_entry(APP_BUS_NAME)?,
        )?;
        let portal = data.join(SETTINGS_DIR).join(PORTAL_FILE_NAME);
        Self::write(&portal, &Self::portal_entry())?;
        Ok(Self::instructions(&portal))
    }
}
