use gpui::{AnyElement, Context, IntoElement, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::theme::colors::{ACCENT, DANGER, FG_BRIGHT};
use crate::core::theme::metrics::{CONFIRM_GAP, CONTROL_RADIUS, CRUMBS_HEIGHT, FILTER_FIELD_PAD_X};
use crate::features::devices::domain::entities::format_request::Filesystem;
use crate::features::devices::domain::usecases::volume_label::volume_label;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::usb_format::FormatPhase;

impl ShellView {
    pub(super) fn format_label_controls(
        &self,
        filesystem: Filesystem,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        let state = self.usb_format.as_ref()?;
        let normalized = volume_label(filesystem, state.label.read(cx).text());
        let status = match normalized {
            Ok(label) => t!(
                "device.format.label.preview",
                label = if label.is_empty() {
                    t!("device.format.label.unnamed").to_string()
                } else {
                    label
                }
            )
            .to_string(),
            Err(_) => t!("device.format.error.label").to_string(),
        };
        Some(
            div()
                .flex()
                .flex_col()
                .gap(px(CONFIRM_GAP))
                .child(t!("device.format.label.title").to_string())
                .child(
                    div()
                        .flex()
                        .items_center()
                        .h(px(CRUMBS_HEIGHT))
                        .px(px(FILTER_FIELD_PAD_X))
                        .rounded(px(CONTROL_RADIUS))
                        .border_1()
                        .border_color(rgb(ACCENT))
                        .text_color(rgb(FG_BRIGHT))
                        .child(state.label.clone()),
                )
                .child(t!(format!("device.format.label.hint.{}", filesystem.id())).to_string())
                .child(
                    div()
                        .when(!self.format_label_valid(cx), |status| {
                            status.text_color(rgb(DANGER))
                        })
                        .child(status),
                )
                .into_any_element(),
        )
    }

    pub(super) fn format_label_valid(&self, cx: &Context<Self>) -> bool {
        let Some(state) = self.usb_format.as_ref() else {
            return false;
        };
        let FormatPhase::Choose {
            options, selected, ..
        } = &state.phase
        else {
            return false;
        };
        options
            .filesystems
            .get(*selected)
            .is_some_and(|filesystem| {
                volume_label(*filesystem, state.label.read(cx).text()).is_ok()
            })
    }
}
