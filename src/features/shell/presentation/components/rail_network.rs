use std::iter;
use std::rc::Rc;

use gpui::{AnyElement, IntoElement};

use crate::core::enums::assets::icon::Icon;
use crate::features::shell::presentation::components::rail::{Rail, trailing_icon};
use crate::features::shell::presentation::state::rail_stop::RailStop;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::rail::rail_device_item::RailDeviceItem;
use crate::global_components::rail::rail_section_label::RailSectionLabel;

const NETWORK_ID: &str = "network";
const NETWORK_SECTION_KEY: &str = "rail.section.network";

impl Rail {
    pub(super) fn network_section(&self) -> Vec<AnyElement> {
        if self.network.is_empty() {
            return Vec::new();
        }
        let label = RailSectionLabel::new(NETWORK_SECTION_KEY, self.motion).into_any_element();
        let items = self.network.iter().enumerate().map(|(index, mount)| {
            let on_unmount = self.on_unmount.clone();
            RailDeviceItem {
                index,
                id: NETWORK_ID,
                icon: Icon::Network,
                label: mount.label.clone(),
                trailing: trailing_icon(mount),
                protocol: mount.protocol,
                active: mount.active,
                cursor: self.cursor == Some(RailStop::Network(index)),
                motion: self.motion,
                on_select: self.on_network.clone(),
                on_eject: Rc::new(move |window, cx| on_unmount(index, window, cx)),
                on_context: None,
                on_drop: None,
                on_edit: (mount.saved && !mount.mounted).then(|| {
                    let on_edit = self.on_edit_network.clone();
                    let edit: ClickHandler = Rc::new(move |window, cx| on_edit(index, window, cx));
                    edit
                }),
            }
            .into_any_element()
        });
        iter::once(label).chain(items).collect()
    }
}
