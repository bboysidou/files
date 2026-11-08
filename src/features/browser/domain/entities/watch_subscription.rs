use std::any::Any;

use futures::channel::mpsc::UnboundedReceiver;

pub struct WatchSubscription {
    pub changes: UnboundedReceiver<()>,
    pub guard: Box<dyn Any + Send>,
}
