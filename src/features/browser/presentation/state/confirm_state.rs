use std::path::PathBuf;

use crate::features::operations::domain::entities::run_plan::RunPlan;
use crate::features::operations::domain::entities::transfer_request::TransferRequest;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConfirmState {
    Purge(Vec<PathBuf>),
    DeleteForever(Vec<PathBuf>),
    EmptyTrash,
    Collision(TransferRequest, usize),
    Run(RunPlan),
}
