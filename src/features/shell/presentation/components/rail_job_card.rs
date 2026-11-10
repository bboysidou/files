use gpui::{App, FontWeight, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};

use crate::core::enums::assets::icon::Icon;
use crate::core::theme::colors::{CARD_SUNKEN, FG_BRIGHT, FG_MUTED, FG_SOFT, LINE};
use crate::core::theme::metrics::{
    CARD_RADIUS, JOB_CARD_GAP, JOB_CARD_PAD_X, JOB_CARD_PAD_Y, TEXT_SMALL, TEXT_STATUS, TEXT_TINY,
};
use crate::features::browser::presentation::state::job_card_model::JobCardModel;
use crate::global_components::handlers::ClickHandler;
use crate::global_components::icon_button::IconButton;
use crate::global_components::progress_bar::ProgressBar;

#[derive(IntoElement)]
pub struct RailJobCard {
    pub model: JobCardModel,
    pub on_cancel: Option<ClickHandler>,
}

impl RenderOnce for RailJobCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let progress = div()
            .flex()
            .items_center()
            .gap(px(JOB_CARD_GAP))
            .mt(px(JOB_CARD_GAP))
            .child(ProgressBar {
                fraction: self.model.fraction,
            })
            .when_some(self.on_cancel, |progress, on_cancel| {
                progress.child(IconButton::new("job-cancel", Icon::Close).on_click(on_cancel))
            });
        div()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .px(px(JOB_CARD_PAD_X))
            .py(px(JOB_CARD_PAD_Y))
            .rounded(px(CARD_RADIUS))
            .bg(rgb(CARD_SUNKEN))
            .border_1()
            .border_color(rgb(LINE))
            .child(
                div()
                    .text_size(px(TEXT_TINY))
                    .text_color(rgb(FG_MUTED))
                    .child(self.model.title),
            )
            .child(
                div()
                    .text_size(px(TEXT_SMALL))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(FG_BRIGHT))
                    .overflow_hidden()
                    .text_ellipsis()
                    .whitespace_nowrap()
                    .child(self.model.item),
            )
            .when(!self.model.detail.is_empty(), |card| {
                card.child(
                    div()
                        .text_size(px(TEXT_STATUS))
                        .text_color(rgb(FG_SOFT))
                        .child(self.model.detail),
                )
            })
            .child(progress)
    }
}
