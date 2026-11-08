use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::core::constants::settings::{
    SETTINGS_BACKUP_EXTENSION, SETTINGS_DIR, SETTINGS_FILE, SETTINGS_TEMP_EXTENSION,
};
use crate::features::settings::domain::entities::settings::Settings;
use crate::features::settings::domain::repositories::settings_repository::SettingsRepository;

pub struct TomlSettingsRepository {
    path: Option<PathBuf>,
}

impl TomlSettingsRepository {
    pub fn new() -> Self {
        Self {
            path: dirs::config_dir().map(|config| config.join(SETTINGS_DIR).join(SETTINGS_FILE)),
        }
    }

    fn set_aside(path: &PathBuf) {
        let _ = fs::rename(path, path.with_extension(SETTINGS_BACKUP_EXTENSION));
    }
}

impl SettingsRepository for TomlSettingsRepository {
    fn load(&self) -> Settings {
        let Some(path) = &self.path else {
            return Settings::default();
        };
        match fs::read_to_string(path) {
            Ok(text) => toml::from_str(&text).unwrap_or_else(|_| {
                Self::set_aside(path);
                Settings::default()
            }),
            Err(error) if error.kind() == ErrorKind::NotFound => Settings::default(),
            Err(_) => Settings::default(),
        }
    }

    fn save(&self, settings: &Settings) {
        let (Some(path), Ok(text)) = (&self.path, toml::to_string_pretty(settings)) else {
            return;
        };
        let Some(directory) = path.parent() else {
            return;
        };
        let temporary = path.with_extension(SETTINGS_TEMP_EXTENSION);
        let written = fs::create_dir_all(directory)
            .and_then(|()| fs::write(&temporary, text))
            .and_then(|()| fs::rename(&temporary, path));
        if written.is_err() {
            let _ = fs::remove_file(&temporary);
        }
    }
}
