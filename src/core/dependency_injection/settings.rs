use std::sync::Arc;

use crate::features::settings::data::repositories::toml_settings_repository::TomlSettingsRepository;
use crate::features::settings::domain::repositories::settings_repository::SettingsRepository;

pub fn settings_repository() -> Arc<dyn SettingsRepository> {
    Arc::new(TomlSettingsRepository::new())
}
