use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::helpers::format::file_size::file_size;
use crate::core::theme::colors::{CARD, FG_BRIGHT, FG_MUTED, LINE};
use crate::core::theme::metrics::{
    CARD_RADIUS, OVERVIEW_CARD_PAD, OVERVIEW_CARD_WIDTH, OVERVIEW_ITEM_GAP, TEXT_STATUS,
};
use crate::features::overview::domain::entities::overview_report::DiskUsage;
use crate::global_components::progress_bar::ProgressBar;

#[derive(IntoElement)]
pub struct DiskCard {
    pub disk: DiskUsage,
}

impl RenderOnce for DiskCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let fraction = self.disk.used as f32 / self.disk.total.max(1) as f32;
        let used = t!(
            "overview.used",
            used = file_size(self.disk.used),
            total = file_size(self.disk.total)
        )
        .to_string();
        div()
            .flex()
            .flex_col()
            .gap(px(OVERVIEW_ITEM_GAP))
            .w(px(OVERVIEW_CARD_WIDTH))
            .p(px(OVERVIEW_CARD_PAD))
            .rounded(px(CARD_RADIUS))
            .bg(rgb(CARD))
            .border_1()
            .border_color(rgb(LINE))
            .child(
                div()
                    .text_color(rgb(FG_BRIGHT))
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(self.disk.mount.to_string_lossy().into_owned()),
            )
            .child(div().flex().child(ProgressBar { fraction }))
            .child(
                div()
                    .text_size(px(TEXT_STATUS))
                    .text_color(rgb(FG_MUTED))
                    .child(used),
            )
    }
}
