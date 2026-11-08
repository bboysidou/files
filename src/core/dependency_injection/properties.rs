use std::sync::Arc;

use crate::features::properties::data::repositories::local_properties_source::LocalPropertiesSource;
use crate::features::properties::domain::repositories::properties_source::PropertiesSource;

pub fn properties_source() -> Arc<dyn PropertiesSource> {
    Arc::new(LocalPropertiesSource)
}
