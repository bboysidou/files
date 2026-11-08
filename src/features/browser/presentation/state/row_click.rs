use gpui::{ClickEvent, Modifiers};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RowClick {
    pub index: usize,
    pub modifiers: Modifiers,
    pub count: usize,
}

impl RowClick {
    pub fn from_event(index: usize, event: &ClickEvent) -> Self {
        Self {
            index,
            modifiers: event.modifiers(),
            count: event.click_count(),
        }
    }
}
