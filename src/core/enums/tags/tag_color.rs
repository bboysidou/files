use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TagColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Grey,
}

impl TagColor {
    pub const ALL: [TagColor; 7] = [
        TagColor::Red,
        TagColor::Orange,
        TagColor::Yellow,
        TagColor::Green,
        TagColor::Blue,
        TagColor::Purple,
        TagColor::Grey,
    ];

    pub fn hex(self) -> u32 {
        match self {
            TagColor::Red => 0xF44747,
            TagColor::Orange => 0xCE9178,
            TagColor::Yellow => 0xD7BA7D,
            TagColor::Green => 0x4EC9B0,
            TagColor::Blue => 0x569CD6,
            TagColor::Purple => 0xC586C0,
            TagColor::Grey => 0xA0A0A0,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            TagColor::Red => "Red",
            TagColor::Orange => "Orange",
            TagColor::Yellow => "Yellow",
            TagColor::Green => "Green",
            TagColor::Blue => "Blue",
            TagColor::Purple => "Purple",
            TagColor::Grey => "Grey",
        }
    }

    pub fn from_names(names: &[String]) -> Vec<TagColor> {
        Self::ALL
            .into_iter()
            .filter(|tag| names.iter().any(|name| Self::from_name(name) == Some(*tag)))
            .collect()
    }

    pub fn from_name(name: &str) -> Option<TagColor> {
        Self::ALL
            .into_iter()
            .find(|tag| tag.name().eq_ignore_ascii_case(name.trim()))
    }
}
