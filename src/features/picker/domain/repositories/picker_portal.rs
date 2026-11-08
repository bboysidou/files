use std::any::Any;

use futures::channel::mpsc::UnboundedSender;
use futures::channel::oneshot;

use crate::features::picker::domain::entities::picker_request::{PickerRequest, PickerResult};

pub type PickerReply = oneshot::Sender<PickerResult>;

pub trait PickerPortal: Send + Sync {
    fn serve(&self, sender: UnboundedSender<(PickerRequest, PickerReply)>) -> Option<Box<dyn Any>>;
}
