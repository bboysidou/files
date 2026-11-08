use crate::core::enums::assets::icon::Icon;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewMode {
    List,
    Columns,
    Grid,
    Dual,
}

impl ViewMode {
    pub const ALL: [ViewMode; 4] = [
        ViewMode::List,
        ViewMode::Columns,
        ViewMode::Grid,
        ViewMode::Dual,
    ];

    pub fn icon(self) -> Icon {
        match self {
            ViewMode::List => Icon::List,
            ViewMode::Columns => Icon::Columns,
            ViewMode::Grid => Icon::Grid,
            ViewMode::Dual => Icon::Dual,
        }
    }
}
