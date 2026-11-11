use crate::features::shell::presentation::state::device_item::DeviceItem;
use crate::global_components::handlers::{IndexHandler, IndexPathsHandler};

pub struct FavoriteRail {
    pub items: Vec<DeviceItem>,
    pub on_select: IndexHandler,
    pub on_remove: IndexHandler,
    pub on_drop: IndexPathsHandler,
}
