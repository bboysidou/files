use std::time::Duration;

use gpui::{Context, IntoElement, div, prelude::*};
use rust_i18n::t;

use crate::features::browser::presentation::state::job_card_model::JobCardModel;
use crate::features::devices::domain::entities::format_request::FormatTarget;
use crate::features::shell::presentation::components::rail_job_card::RailJobCard;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::usb_format::FormatPhase;

impl ShellView {
    pub(super) fn watch_format_progress(&mut self, target: FormatTarget, cx: &mut Context<Self>) {
        let repository = self.device_repository.clone();
        self.format_progress_task = Some(cx.spawn(async move |this, cx| {
            loop {
                let repository = repository.clone();
                let target = target.clone();
                let progress = cx
                    .background_executor()
                    .spawn(async move { repository.format_progress(&target) })
                    .await;
                let running = this
                    .update(cx, |this, cx| {
                        if let Some(state) = this.usb_format.as_mut()
                            && let FormatPhase::Running { progress: current } = &mut state.phase
                        {
                            *current = progress;
                            cx.notify();
                            return true;
                        }
                        false
                    })
                    .unwrap_or(false);
                if !running {
                    break;
                }
                cx.background_executor()
                    .timer(Duration::from_millis(500))
                    .await;
            }
        }));
    }

    pub(super) fn format_progress_card(&self, progress: Option<f32>) -> gpui::AnyElement {
        let detail = match progress {
            Some(fraction) => t!(
                "device.format.progress.value",
                percent = format!("{:.0}", fraction * 100.0)
            )
            .to_string(),
            None => String::new(),
        };
        div()
            .flex()
            .flex_col()
            .child(t!("device.format.running").to_string())
            .child(RailJobCard {
                model: JobCardModel {
                    title: t!("device.format.progress.title").to_string().into(),
                    item: self
                        .usb_format
                        .as_ref()
                        .map(|state| state.device.label.clone())
                        .unwrap_or_default()
                        .into(),
                    detail: detail.into(),
                    fraction: progress,
                },
                on_cancel: None,
            })
            .into_any_element()
    }
}
