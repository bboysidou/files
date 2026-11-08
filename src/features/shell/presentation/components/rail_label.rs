use gpui::{
    AnimationExt, App, ElementId, IntoElement, RenderOnce, SharedString, Window, div, prelude::*,
    px,
};

use crate::core::theme::metrics::RAIL_LABEL_SLIDE;
use crate::features::shell::presentation::state::rail_motion::RailMotion;

#[derive(IntoElement)]
pub struct RailLabel {
    id: &'static str,
    text: SharedString,
    motion: RailMotion,
}

impl RailLabel {
    pub fn new(id: &'static str, text: impl Into<SharedString>, motion: RailMotion) -> Self {
        Self {
            id,
            text: text.into(),
            motion,
        }
    }
}

impl RenderOnce for RailLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let label = div()
            .relative()
            .flex_1()
            .min_w_0()
            .overflow_hidden()
            .whitespace_nowrap()
            .text_ellipsis()
            .child(self.text);
        if !self.motion.animating() {
            return label.into_any_element();
        }
        let id = ElementId::NamedInteger(self.id.into(), self.motion.generation as u64);
        label
            .with_animation(id, self.motion.animation(), |label, progress| {
                label
                    .opacity(progress)
                    .left(px(-RAIL_LABEL_SLIDE * (1.0 - progress)))
            })
            .into_any_element()
    }
}
