use gpui::Task;

use crate::features::overview::domain::entities::overview_report::OverviewReport;

#[derive(Default)]
pub struct OverviewState {
    pub report: Option<OverviewReport>,
    pub task: Option<Task<()>>,
}
