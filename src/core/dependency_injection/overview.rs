use std::sync::Arc;

use crate::features::overview::data::repositories::system_overview_source::SystemOverviewSource;
use crate::features::overview::domain::repositories::overview_source::OverviewSource;

pub fn overview_source() -> Arc<dyn OverviewSource> {
    Arc::new(SystemOverviewSource)
}
