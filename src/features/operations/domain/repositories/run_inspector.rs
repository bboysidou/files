use std::path::Path;

use crate::features::operations::domain::entities::run_plan::RunPlan;

pub trait RunInspector: Send + Sync {
    fn inspect(&self, path: &Path) -> RunPlan;
}
