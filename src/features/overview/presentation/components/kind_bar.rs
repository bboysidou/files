use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, relative, rgb};
use rust_i18n::t;

use crate::core::helpers::files::extension_color::extension_color;
use crate::core::helpers::format::file_size::file_size;
use crate::core::theme::colors::{FG_MUTED, LINE};
use crate::core::theme::metrics::{
    OVERVIEW_BAR_HEIGHT, OVERVIEW_ITEM_GAP, OVERVIEW_LABEL_WIDTH, OVERVIEW_ROW_HEIGHT,
    OVERVIEW_VALUE_WIDTH,
};
use crate::features::overview::domain::entities::overview_report::KindUsage;

#[derive(IntoElement)]
pub struct KindBar {
    pub usage: KindUsage,
    pub largest: u64,
}

impl RenderOnce for KindBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let key = self.usage.kind.label_key();
        let color = extension_color(Some(key));
        let fraction = self.usage.bytes as f32 / self.largest.max(1) as f32;
        div()
            .flex()
            .items_center()
            .gap(px(OVERVIEW_ITEM_GAP))
            .h(px(OVERVIEW_ROW_HEIGHT))
            .child(div().w(px(OVERVIEW_LABEL_WIDTH)).child(t!(key).to_string()))
            .child(
                div()
                    .flex_1()
                    .h(px(OVERVIEW_BAR_HEIGHT))
                    .rounded_full()
                    .bg(rgb(LINE))
                    .child(
                        div()
                            .h_full()
                            .w(relative(fraction))
                            .rounded_full()
                            .bg(color),
                    ),
            )
            .child(
                div()
                    .w(px(OVERVIEW_VALUE_WIDTH))
                    .flex()
                    .justify_end()
                    .text_color(rgb(FG_MUTED))
                    .child(file_size(self.usage.bytes)),
            )
    }
}
