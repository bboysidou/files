use std::time::SystemTime;

use gpui::{AnyElement, App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::theme::colors::{FG_DIM, FG_MUTED};
use crate::core::theme::metrics::{
    OVERVIEW_ITEM_GAP, OVERVIEW_PAD, OVERVIEW_SECTION_GAP, TEXT_STATUS, TEXT_TINY,
};
use crate::features::overview::domain::entities::overview_report::OverviewReport;
use crate::features::overview::presentation::components::disk_card::DiskCard;
use crate::features::overview::presentation::components::kind_bar::KindBar;
use crate::features::overview::presentation::components::recent_row::RecentRow;
use crate::global_components::handlers::IndexHandler;

#[derive(IntoElement)]
pub struct OverviewPage {
    pub report: Option<OverviewReport>,
    pub on_recent: IndexHandler,
}

fn section(title_key: &str, note: Option<String>, body: AnyElement) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap(px(OVERVIEW_ITEM_GAP))
        .child(
            div()
                .flex()
                .gap(px(OVERVIEW_ITEM_GAP))
                .text_size(px(TEXT_TINY))
                .text_color(rgb(FG_DIM))
                .child(t!(title_key).to_string())
                .children(note),
        )
        .child(body)
        .into_any_element()
}

fn muted(text: String) -> AnyElement {
    div()
        .text_size(px(TEXT_STATUS))
        .text_color(rgb(FG_MUTED))
        .child(text)
        .into_any_element()
}

impl RenderOnce for OverviewPage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let page = div()
            .id("overview")
            .size_full()
            .overflow_y_scroll()
            .p(px(OVERVIEW_PAD))
            .flex()
            .flex_col()
            .gap(px(OVERVIEW_SECTION_GAP));
        let Some(report) = self.report else {
            return page.child(muted(t!("overview.scanning").to_string()));
        };
        let largest = report.kinds.first().map_or(0, |usage| usage.bytes);
        let disks = div()
            .flex()
            .flex_wrap()
            .gap(px(OVERVIEW_ITEM_GAP))
            .children(report.disks.into_iter().map(|disk| DiskCard { disk }));
        let kinds = div().flex().flex_col().children(
            report
                .kinds
                .into_iter()
                .map(|usage| KindBar { usage, largest }),
        );
        let now = SystemTime::now();
        let recent: AnyElement =
            if report.recent.is_empty() {
                muted(t!("overview.no_recent").to_string())
            } else {
                div()
                    .flex()
                    .flex_col()
                    .children(report.recent.into_iter().enumerate().map(|(index, file)| {
                        RecentRow {
                            index,
                            file,
                            now,
                            on_select: self.on_recent.clone(),
                        }
                    }))
                    .into_any_element()
            };
        let scanned = t!("overview.scanned", count = report.scanned).to_string();
        page.child(section("overview.disks", None, disks.into_any_element()))
            .child(section(
                "overview.home",
                Some(scanned),
                kinds.into_any_element(),
            ))
            .child(section("overview.recent", None, recent))
    }
}
