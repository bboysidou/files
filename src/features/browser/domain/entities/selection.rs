use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Selection {
    cursor: usize,
    anchor: usize,
    marked: BTreeSet<usize>,
}

impl Selection {
    pub fn at(cursor: usize) -> Self {
        Self {
            cursor,
            anchor: cursor,
            marked: BTreeSet::new(),
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn is_marked(&self, index: usize) -> bool {
        self.marked.contains(&index)
    }

    pub fn marked_count(&self) -> usize {
        self.marked.len()
    }

    pub fn marked(&self) -> impl Iterator<Item = usize> + '_ {
        self.marked.iter().copied()
    }

    pub fn set_marked(&mut self, marked: impl IntoIterator<Item = usize>) {
        self.marked = marked.into_iter().collect();
    }

    pub fn invert(&mut self, len: usize) {
        self.marked = (0..len)
            .filter(|index| !self.marked.contains(index))
            .collect();
    }

    pub fn mark_all(&mut self, len: usize) {
        self.marked = (0..len).collect();
    }

    pub fn move_to(&mut self, index: usize) {
        self.cursor = index;
        self.anchor = index;
        self.marked.clear();
    }

    pub fn toggle(&mut self, index: usize) {
        if !self.marked.remove(&index) {
            self.marked.insert(index);
        }
        self.cursor = index;
        self.anchor = index;
    }

    pub fn extend_to(&mut self, index: usize) {
        let (low, high) = (self.anchor.min(index), self.anchor.max(index));
        self.marked = (low..=high).collect();
        self.cursor = index;
    }
}
