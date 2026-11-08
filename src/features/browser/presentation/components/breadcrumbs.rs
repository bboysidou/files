use gpui::{AnyElement, App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::constants::paths::PATH_SEPARATOR;
use crate::core::enums::places::crumb_root::CrumbRoot;
use crate::core::theme::colors::{CARD, CRUMB_SEPARATOR, LINE};
use crate::core::theme::metrics::{
    CONTROL_RADIUS, CRUMBS_GAP, CRUMBS_HEIGHT, CRUMBS_PAD_LEFT, CRUMBS_PAD_RIGHT,
};
use crate::features::browser::presentation::components::crumb_segment::CrumbSegment;
use crate::features::places::domain::entities::crumb_trail::CrumbTrail;
use crate::global_components::handlers::IndexHandler;
use crate::global_components::kbd_hint::KbdHint;

const PATH_EDIT_KEY: &str = ":";

#[derive(IntoElement)]
pub struct Breadcrumbs {
    pub trail: CrumbTrail,
    pub on_crumb: IndexHandler,
}

impl Breadcrumbs {
    fn separator() -> AnyElement {
        div()
            .flex_shrink_0()
            .text_color(rgb(CRUMB_SEPARATOR))
            .child(PATH_SEPARATOR)
            .into_any_element()
    }

    fn crumbs(&self) -> Vec<AnyElement> {
        let count = self.trail.segments.len();
        let root = CrumbSegment {
            index: 0,
            text: self.trail.root.text().into_owned().into(),
            last: count == 0,
            on_click: self.on_crumb.clone(),
        };
        let mut crumbs = vec![root.into_any_element()];
        for (offset, segment) in self.trail.segments.iter().enumerate() {
            let skip_separator = offset == 0 && self.trail.root == CrumbRoot::Filesystem;
            if !skip_separator {
                crumbs.push(Self::separator());
            }
            let crumb = CrumbSegment {
                index: offset + 1,
                text: segment.clone().into(),
                last: offset + 1 == count,
                on_click: self.on_crumb.clone(),
            };
            crumbs.push(crumb.into_any_element());
        }
        crumbs
    }
}

impl RenderOnce for Breadcrumbs {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_1()
            .min_w_0()
            .items_center()
            .gap(px(CRUMBS_GAP))
            .h(px(CRUMBS_HEIGHT))
            .pl(px(CRUMBS_PAD_LEFT))
            .pr(px(CRUMBS_PAD_RIGHT))
            .rounded(px(CONTROL_RADIUS))
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(LINE))
            .overflow_hidden()
            .whitespace_nowrap()
            .children(self.crumbs())
            .child(div().ml_auto().child(KbdHint::new(PATH_EDIT_KEY)))
    }
}
