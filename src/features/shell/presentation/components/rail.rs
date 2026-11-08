use std::iter;

use gpui::{
    AnimationExt, AnyElement, App, ElementId, IntoElement, RenderOnce, Window, div, prelude::*, px,
};

use crate::core::helpers::motion::lerp::lerp;
use crate::core::theme::metrics::{RAIL_PAD_BOTTOM, RAIL_PAD_TOP, RAIL_PAD_X};
use crate::features::places::domain::entities::place::{Place, PlaceSection};
use crate::features::shell::presentation::components::rail_brand::RailBrand;
use crate::features::shell::presentation::components::rail_nav_item::RailNavItem;
use crate::features::shell::presentation::components::rail_search::RailSearch;
use crate::features::shell::presentation::components::rail_section_label::RailSectionLabel;
use crate::features::shell::presentation::components::rail_tags::RailTags;
use crate::features::shell::presentation::state::rail_motion::RailMotion;
use crate::global_components::handlers::{ClickHandler, IndexHandler};

const TAGS_SECTION_KEY: &str = "rail.section.tags";

#[derive(IntoElement)]
pub struct Rail {
    pub places: Vec<Place>,
    pub active: Option<usize>,
    pub motion: RailMotion,
    pub on_toggle: ClickHandler,
    pub on_select: IndexHandler,
    pub on_search: ClickHandler,
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
                    motion: self.motion,
                    on_select: self.on_select.clone(),
                }
                .into_any_element()
            });
        iter::once(label).chain(items).collect()
    }
}

impl RenderOnce for Rail {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let motion = self.motion;
        let sections: Vec<AnyElement> = PlaceSection::ALL
            .iter()
            .flat_map(|section| self.section(*section))
            .collect();
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
            .child(RailSearch {
                motion,
                on_search: self.on_search,
            })
            .children(sections)
            .child(RailSectionLabel::new(TAGS_SECTION_KEY, motion))
            .child(RailTags { open: motion.open });
        let rail = div()
            .id("rail")
            .flex_shrink_0()
            .h_full()
            .w(px(motion.width()))
            .overflow_hidden()
            .child(content);
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
