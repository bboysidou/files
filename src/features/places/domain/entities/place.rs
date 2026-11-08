use std::path::PathBuf;

use crate::core::enums::places::place_kind::PlaceKind;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlaceSection {
    Places,
    Devices,
}

impl PlaceSection {
    pub const ALL: [PlaceSection; 2] = [PlaceSection::Places, PlaceSection::Devices];

    pub fn label_key(self) -> &'static str {
        match self {
            PlaceSection::Places => "rail.section.places",
            PlaceSection::Devices => "rail.section.devices",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PlaceTarget {
    Overview,
    Directory(PathBuf),
    Trash,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Place {
    pub kind: PlaceKind,
    pub section: PlaceSection,
    pub target: PlaceTarget,
}
