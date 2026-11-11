use std::iter;
use std::rc::Rc;

use gpui::{AnyElement, IntoElement};

use crate::core::enums::assets::icon::Icon;
use crate::features::shell::presentation::components::rail::{Rail, place_drop};
use crate::features::shell::presentation::components::rail_device_item::RailDeviceItem;
use crate::features::shell::presentation::components::rail_section_label::RailSectionLabel;
use crate::features::shell::presentation::state::rail_stop::RailStop;

const FAVORITE_ID: &str = "favorite";
const FAVORITES_SECTION_KEY: &str = "rail.section.favorites";

impl Rail {
    pub(super) fn favorites_section(&self) -> Vec<AnyElement> {
        if self.favorites.items.is_empty() {
            return Vec::new();
        }
        let label = RailSectionLabel::new(FAVORITES_SECTION_KEY, self.motion).into_any_element();
        let items = self
            .favorites
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let on_remove = self.favorites.on_remove.clone();
                RailDeviceItem {
                    index,
                    id: FAVORITE_ID,
                    icon: Icon::Folder,
                    label: item.label.clone(),
                    trailing: Some(Icon::Close),
                    protocol: None,
                    active: item.active,
                    cursor: self.cursor == Some(RailStop::Favorite(index)),
                    motion: self.motion,
                    on_select: self.favorites.on_select.clone(),
                    on_eject: Rc::new(move |window, cx| on_remove(index, window, cx)),
                    on_edit: None,
                    on_context: None,
                    on_drop: Some(place_drop(self.favorites.on_drop.clone(), index)),
                }
                .into_any_element()
            });
        iter::once(label).chain(items).collect()
    }
}
