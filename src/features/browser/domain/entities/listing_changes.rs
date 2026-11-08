#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ListingChanges {
    pub added: usize,
    pub removed: usize,
    pub changed: usize,
}

impl ListingChanges {
    pub fn is_empty(self) -> bool {
        self.added == 0 && self.removed == 0 && self.changed == 0
    }
}
