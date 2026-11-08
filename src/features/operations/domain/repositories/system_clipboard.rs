use crate::features::operations::domain::entities::clipboard::Clipboard;

pub trait SystemClipboard: Send + Sync {
    fn write(&self, clipboard: &Clipboard) -> bool;
    fn read(&self) -> Option<Clipboard>;
    fn clear(&self);
}
