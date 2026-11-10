use std::iter;
use std::rc::Rc;

use gpui::FocusHandle;
use gpui::{
    AnimationExt, AnyElement, App, ElementId, IntoElement, RenderOnce, Window, div, prelude::*, px,
};

use crate::core::enums::assets::icon::Icon;
use crate::core::helpers::motion::lerp::lerp;
use crate::core::theme::metrics::{
    JOB_SECTION_PAD_TOP, RAIL_PAD_BOTTOM, RAIL_PAD_TOP, RAIL_PAD_X, RAIL_PILL_GAP,
};
use crate::features::browser::presentation::state::job_card_model::JobCardModel;
use crate::features::places::domain::entities::place::{Place, PlaceSection, PlaceTarget};
use crate::features::shell::presentation::components::rail_brand::RailBrand;
use crate::features::shell::presentation::components::rail_device_item::RailDeviceItem;
use crate::features::shell::presentation::components::rail_group::{PHONES, RailGroup, SSH};
use crate::features::shell::presentation::components::rail_job_card::RailJobCard;
use crate::features::shell::presentation::components::rail_nav_item::RailNavItem;
use crate::features::shell::presentation::components::rail_pill::{
    CONNECT_PILL, RailPill, SEARCH_PILL,
};
use crate::features::shell::presentation::components::rail_section_label::RailSectionLabel;
use crate::features::shell::presentation::components::rail_tags::RailTags;
use crate::features::shell::presentation::state::device_item::DeviceItem;
use crate::features::shell::presentation::state::rail_motion::RailMotion;
use crate::features::shell::presentation::state::rail_stop::RailStop;
use crate::global_components::handlers::{
    ClickHandler, IndexHandler, IndexPathsHandler, PathsHandler,
};

const TAGS_SECTION_KEY: &str = "rail.section.tags";
const DEVICE_ID: &str = "device";
const NETWORK_ID: &str = "network";
const NETWORK_SECTION_KEY: &str = "rail.section.network";
pub const RAIL_CONTEXT: &str = "Rail";

#[derive(IntoElement)]
pub struct Rail {
    pub places: Vec<Place>,
    pub active: Option<usize>,
    pub motion: RailMotion,
    pub on_toggle: ClickHandler,
    pub on_select: IndexHandler,
    pub on_drop_place: IndexPathsHandler,
    pub on_search: ClickHandler,
    pub on_connect: ClickHandler,
    pub devices: Vec<DeviceItem>,
    pub on_device: IndexHandler,
    pub on_device_context: crate::global_components::handlers::IndexPointHandler,
    pub on_eject: IndexHandler,
    pub phones: Vec<DeviceItem>,
    pub on_phone: IndexHandler,
    pub on_phone_eject: IndexHandler,
    pub ssh: Vec<DeviceItem>,
    pub on_ssh: IndexHandler,
    pub on_ssh_eject: IndexHandler,
    pub network: Vec<DeviceItem>,
    pub on_network: IndexHandler,
    pub on_unmount: IndexHandler,
    pub on_edit_network: IndexHandler,
    pub job: Option<JobCardModel>,
    pub on_cancel_job: ClickHandler,
    pub on_tag: IndexHandler,
    pub focus: FocusHandle,
    pub cursor: Option<RailStop>,
}

impl Rail {
    fn section(&self, section: PlaceSection) -> Vec<AnyElement> {
        let label = RailSectionLabel::new(section.label_key(), self.motion).into_any_element();
        let items = self
            .places
            .iter()
            .enumerate()
            .filter(|(_, place)| place.section == section)
            .map(|(index, place)| {
                RailNavItem {
                    index,
                    kind: place.kind,
                    active: Some(index) == self.active,
                    cursor: self.cursor == Some(RailStop::Place(index)),
                    motion: self.motion,
                    on_select: self.on_select.clone(),
                    on_drop: matches!(place.target, PlaceTarget::Directory(_))
                        .then(|| place_drop(self.on_drop_place.clone(), index)),
                }
                .into_any_element()
            });
        let devices = if section == PlaceSection::Devices {
            self.device_items()
        } else {
            Vec::new()
        };
        iter::once(label).chain(items).chain(devices).collect()
    }
}

impl Rail {
    fn device_items(&self) -> Vec<AnyElement> {
        self.devices
            .iter()
            .enumerate()
            .map(|(index, device)| {
                let on_eject = self.on_eject.clone();
                RailDeviceItem {
                    index,
                    id: DEVICE_ID,
                    icon: Icon::Usb,
                    label: device.label.clone(),
                    trailing: device.mounted.then_some(Icon::Eject),
                    protocol: None,
                    active: device.active,
                    cursor: self.cursor == Some(RailStop::Device(index)),
                    motion: self.motion,
                    on_select: self.on_device.clone(),
                    on_eject: Rc::new(move |window, cx| on_eject(index, window, cx)),
                    on_edit: None,
                    on_context: Some({
                        let handler = self.on_device_context.clone();
                        Rc::new(move |position, window, cx| handler(index, position, window, cx))
                    }),
                }
                .into_any_element()
            })
            .collect()
    }

    fn group(
        &self,
        group: RailGroup,
        items: &[DeviceItem],
        (on_select, on_eject): (&IndexHandler, &IndexHandler),
    ) -> Vec<AnyElement> {
        if items.is_empty() {
            return Vec::new();
        }
        let label = RailSectionLabel::new(group.label_key, self.motion).into_any_element();
        let rows = items.iter().enumerate().map(|(index, item)| {
            let on_eject = on_eject.clone();
            RailDeviceItem {
                index,
                id: group.id,
                icon: group.icon,
                label: item.label.clone(),
                trailing: item.mounted.then_some(Icon::Eject),
                protocol: None,
                active: item.active,
                cursor: self.cursor == Some((group.stop)(index)),
                motion: self.motion,
                on_select: on_select.clone(),
                on_eject: Rc::new(move |window, cx| on_eject(index, window, cx)),
                on_edit: None,
                on_context: None,
            }
            .into_any_element()
        });
        iter::once(label).chain(rows).collect()
    }

    fn network_section(&self) -> Vec<AnyElement> {
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

impl Rail {
    fn tags_section(&self) -> [AnyElement; 2] {
        let cursor = match self.cursor {
            Some(RailStop::Tag(index)) => Some(index),
            _ => None,
        };
        [
            RailSectionLabel::new(TAGS_SECTION_KEY, self.motion).into_any_element(),
            RailTags {
                open: self.motion.open,
                cursor,
                on_select: self.on_tag.clone(),
            }
            .into_any_element(),
        ]
    }

    fn job_section(&self) -> Option<AnyElement> {
        let model = self.job.clone().filter(|_| self.motion.open)?;
        Some(
            div()
                .mt_auto()
                .pt(px(JOB_SECTION_PAD_TOP))
                .child(RailJobCard {
                    model,
                    on_cancel: Some(self.on_cancel_job.clone()),
                })
                .into_any_element(),
        )
    }
}

impl Rail {
    fn sections(&self) -> Vec<AnyElement> {
        PlaceSection::ALL
            .iter()
            .flat_map(|section| self.section(*section))
            .chain(self.group(PHONES, &self.phones, (&self.on_phone, &self.on_phone_eject)))
            .chain(self.group(SSH, &self.ssh, (&self.on_ssh, &self.on_ssh_eject)))
            .chain(self.network_section())
            .chain(self.tags_section())
            .collect()
    }
}

impl RenderOnce for Rail {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let motion = self.motion;
        let job = self.job_section();
        let sections = self.sections();
        let content = div()
            .id("rail-content")
            .flex()
            .flex_col()
            .flex_shrink_0()
            .h_full()
            .w(px(motion.width()))
            .overflow_y_scroll()
            .pt(px(RAIL_PAD_TOP))
            .pb(px(RAIL_PAD_BOTTOM))
            .px(px(RAIL_PAD_X))
            .child(RailBrand {
                motion,
                on_toggle: self.on_toggle,
            })
            .child(RailPill {
                spec: SEARCH_PILL,
                motion,
                on_click: self.on_search,
            })
            .child(div().mt(px(RAIL_PILL_GAP)).child(RailPill {
                spec: CONNECT_PILL,
                motion,
                on_click: self.on_connect,
            }))
            .children(sections);
        let rail = div()
            .id("rail")
            .track_focus(&self.focus)
            .key_context(RAIL_CONTEXT)
            .flex_shrink_0()
            .h_full()
            .w(px(motion.width()))
            .overflow_hidden()
            .child(content.children(job));
        if !motion.animating() {
            return rail.into_any_element();
        }
        let (from, to) = (motion.previous_width(), motion.width());
        let id = ElementId::NamedInteger("rail-width".into(), motion.generation as u64);
        rail.with_animation(id, motion.animation(), move |rail, progress| {
            rail.w(px(lerp(from, to, progress)))
        })
        .into_any_element()
    }
}

fn trailing_icon(item: &DeviceItem) -> Option<Icon> {
    if item.mounted {
        return Some(Icon::Eject);
    }
    item.saved.then_some(Icon::Close)
}

fn place_drop(on_drop: IndexPathsHandler, index: usize) -> PathsHandler {
    Rc::new(move |paths, window, cx| on_drop(index, paths, window, cx))
}
