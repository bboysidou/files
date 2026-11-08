use std::path::PathBuf;

use gpui::{App, IntoElement, ObjectFit, RenderOnce, Window, div, img, prelude::*, px, relative};

use rust_i18n::t;

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::metrics::QUICK_LOOK_STAGE_PAD;
use crate::global_components::empty_state::EmptyState;

#[derive(IntoElement)]
pub struct PreviewImage {
    pub path: PathBuf,
    pub zoom: f32,
}

impl RenderOnce for PreviewImage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .p(px(QUICK_LOOK_STAGE_PAD))
            .child(
                img(self.path)
                    .object_fit(ObjectFit::Contain)
                    .flex_shrink_0()
                    .w(relative(self.zoom))
                    .h(relative(self.zoom))
                    .with_fallback(|| {
                        EmptyState {
                            icon: Icon::Image,

                            title: t!("ql.failed.title").to_string().into(),

                            body: t!("ql.failed.body").to_string().into(),
                        }
                        .into_any_element()
                    }),
            )
    }
}
