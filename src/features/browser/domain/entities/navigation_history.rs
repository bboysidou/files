use std::path::PathBuf;

use crate::features::places::domain::entities::place::PlaceTarget;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HistoryStop {
    pub target: PlaceTarget,
    pub focus: Option<PathBuf>,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct NavigationHistory {
    back: Vec<HistoryStop>,
    forward: Vec<HistoryStop>,
}

impl NavigationHistory {
    pub fn from_stops(back: Vec<HistoryStop>, forward: Vec<HistoryStop>) -> Self {
        Self { back, forward }
    }

    pub fn back_stops(&self) -> &[HistoryStop] {
        &self.back
    }

    pub fn forward_stops(&self) -> &[HistoryStop] {
        &self.forward
    }

    pub fn can_go_back(&self) -> bool {
        !self.back.is_empty()
    }

    pub fn can_go_forward(&self) -> bool {
        !self.forward.is_empty()
    }

    pub fn visit(&mut self, leaving: HistoryStop) {
        self.back.push(leaving);
        self.forward.clear();
    }

    pub fn back(&mut self, leaving: HistoryStop) -> Option<HistoryStop> {
        let stop = self.back.pop()?;
        self.forward.push(leaving);
        Some(stop)
    }

    pub fn forward(&mut self, leaving: HistoryStop) -> Option<HistoryStop> {
        let stop = self.forward.pop()?;
        self.back.push(leaving);
        Some(stop)
    }
}
