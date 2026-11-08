use std::path::PathBuf;

use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::theme::colors::SHEET;
use crate::core::theme::metrics::{BODY_PAD_BOTTOM, BODY_PAD_X, SHEET_RADIUS};
use crate::features::places::domain::entities::crumb_trail::CrumbTrail;
use crate::features::places::domain::entities::place::Place;
use crate::features::shell::presentation::components::status_bar::StatusBar;
use crate::features::shell::presentation::components::tab_strip::TabStrip;
use crate::features::shell::presentation::components::toolbar::Toolbar;

#[derive(IntoElement)]
pub struct Sheet {
    pub place: Place,
    pub home: Option<PathBuf>,
}

impl RenderOnce for Sheet {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let trail = CrumbTrail::for_target(&self.place.target, self.home.as_deref());
        let location = trail.display_path();
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .h_full()
            .overflow_hidden()
            .rounded(px(SHEET_RADIUS))
            .bg(rgb(SHEET))
            .child(TabStrip {
                title: t!(self.place.kind.label_key()).to_string().into(),
            })
            .child(Toolbar { trail })
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .px(px(BODY_PAD_X))
                    .pb(px(BODY_PAD_BOTTOM)),
            )
            .child(StatusBar {
                location: location.into(),
            })
    }
}
