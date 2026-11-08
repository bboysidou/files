#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
}
