use crate::features::settings::domain::entities::settings::Settings;

pub trait SettingsRepository: Send + Sync {
    fn load(&self) -> Settings;
    fn save(&self, settings: &Settings);
}
