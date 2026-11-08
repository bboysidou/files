use std::any::Any;

use futures::channel::mpsc::UnboundedSender;

use crate::features::integration::domain::entities::launch_request::LaunchRequest;

pub trait InstanceBridge: Send + Sync {
    fn forward(&self, request: &LaunchRequest) -> bool;
    fn serve(
        &self,
        sender: UnboundedSender<LaunchRequest>,
        claim_file_manager: bool,
    ) -> Option<Box<dyn Any>>;
}
