use std::path::Path;

use crate::features::overview::domain::entities::overview_report::OverviewReport;

pub trait OverviewSource: Send + Sync {
    fn report(&self, home: &Path) -> OverviewReport;
}
