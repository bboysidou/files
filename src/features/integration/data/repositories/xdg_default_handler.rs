use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::core::constants::app::APP_ID;
use crate::core::constants::integration::{
    APPLICATIONS_DIR, DESKTOP_FILE_NAME, DIRECTORY_MIME, PREVIOUS_HANDLER_FILE, XDG_MIME_COMMAND,
};
use crate::core::constants::settings::SETTINGS_DIR;
use crate::features::integration::domain::repositories::default_handler::DefaultHandler;

pub struct XdgDefaultHandler;

impl XdgDefaultHandler {
    fn query() -> Option<String> {
        let output = Command::new(XDG_MIME_COMMAND)
            .args(["query", "default", DIRECTORY_MIME])
            .output()
            .ok()?;
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            .filter(|name| !name.is_empty())
    }

    fn set(desktop: &str) -> Result<(), String> {
        let status = Command::new(XDG_MIME_COMMAND)
            .args(["default", desktop, DIRECTORY_MIME])
            .status()
            .map_err(|error| error.to_string())?;
        status
            .success()
            .then_some(())
            .ok_or_else(|| format!("{XDG_MIME_COMMAND} failed"))
    }

    fn previous_file() -> Option<PathBuf> {
        dirs::config_dir().map(|config| config.join(SETTINGS_DIR).join(PREVIOUS_HANDLER_FILE))
    }

    fn desktop_entry() -> Result<String, String> {
        let executable = env::current_exe().map_err(|error| error.to_string())?;
        Ok(format!(
            "[Desktop Entry]\nType=Application\nName=Files\nComment=File manager\nExec={} %U\nIcon={APP_ID}\nTerminal=false\nStartupWMClass={APP_ID}\nCategories=System;FileTools;FileManager;\nMimeType={DIRECTORY_MIME};\n",
            executable.display()
        ))
    }
}

impl DefaultHandler for XdgDefaultHandler {
    fn is_default(&self) -> bool {
        Self::query().as_deref() == Some(DESKTOP_FILE_NAME)
    }

    fn enable(&self) -> Result<String, String> {
        let applications = dirs::data_dir()
            .ok_or("no data directory")?
            .join(APPLICATIONS_DIR);
        fs::create_dir_all(&applications).map_err(|error| error.to_string())?;
        fs::write(applications.join(DESKTOP_FILE_NAME), Self::desktop_entry()?)
            .map_err(|error| error.to_string())?;
        if let (Some(previous), Some(file)) = (
            Self::query().filter(|name| name != DESKTOP_FILE_NAME),
            Self::previous_file(),
        ) {
            let _ = file.parent().map(fs::create_dir_all);
            fs::write(file, previous).map_err(|error| error.to_string())?;
        }
        Self::set(DESKTOP_FILE_NAME)?;
        Ok(format!("files is now the default for {DIRECTORY_MIME}"))
    }

    fn disable(&self) -> Result<String, String> {
        let previous = Self::previous_file()
            .and_then(|file| fs::read_to_string(file).ok())
            .map(|name| name.trim().to_string());
        match previous.filter(|name| !name.is_empty()) {
            Some(name) => {
                Self::set(&name)?;
                Ok(format!("{name} is the default for {DIRECTORY_MIME} again"))
            }
            None => Err("no previous file manager was recorded".to_string()),
        }
    }
}
