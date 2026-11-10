use crate::features::notifications::domain::entities::desktop_note::DesktopNote;

pub trait DesktopNotifier: Send + Sync {
    fn show(&self, note: &DesktopNote) -> Option<u32>;
    fn close(&self, id: u32);
}
