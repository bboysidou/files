use gpui::{
    AnyElement, Context, Div, Entity, Focusable, IntoElement, Window, div, prelude::*, px, rgb,
};
use rust_i18n::t;

use crate::core::enums::operations::clipboard_mode::ClipboardMode;
use crate::core::enums::operations::collision_policy::CollisionPolicy;
use crate::core::enums::view::view_mode::ViewMode;
use crate::core::theme::colors::{ACCENT, LINE};
use crate::core::theme::metrics::{DUAL_PANE_PAD, DUAL_PANE_RADIUS};
use crate::features::browser::presentation::actions::{
    CopyToOtherPane, MoveToOtherPane, ShowDual, SwitchPane,
};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::operations::domain::entities::transfer_request::TransferRequest;

impl BrowserView {
    pub(super) fn dual_frame(pane: Entity<PaneView>, focused: bool) -> Div {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .h_full()
            .p(px(DUAL_PANE_PAD))
            .rounded(px(DUAL_PANE_RADIUS))
            .border_1()
            .border_color(rgb(if focused { ACCENT } else { LINE }))
            .child(pane)
    }

    fn ensure_partner(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.dual.partner.is_some() {
            return;
        }
        let location = self.tabs[self.active].pane.read(cx).location().clone();
        let partner = self.open_pane(TabSnapshot::fresh(location), window, cx);
        self.dual.partner = Some(partner);
    }

    pub(super) fn pane_body(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let main = self.tabs[self.active].pane.clone();
        if self.options.view != ViewMode::Dual {
            self.dual.focused = false;
            return main.into_any_element();
        }
        self.ensure_partner(window, cx);
        let Some(partner) = self
            .dual
            .partner
            .as_ref()
            .map(|partner| partner.pane.clone())
        else {
            return main.into_any_element();
        };
        if partner.focus_handle(cx).contains_focused(window, cx) {
            self.dual.focused = true;
        } else if main.focus_handle(cx).contains_focused(window, cx) {
            self.dual.focused = false;
        }
        let focused = self.dual.focused;
        self.dual_split([(main, !focused), (partner, focused)], cx)
    }

    fn other_pane(&self) -> Option<Entity<PaneView>> {
        let partner = self.dual.partner.as_ref()?.pane.clone();
        if self.options.view != ViewMode::Dual {
            return None;
        }
        Some(if self.dual.focused {
            self.tabs[self.active].pane.clone()
        } else {
            partner
        })
    }

    fn switch_pane(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(other) = self.other_pane() else {
            return;
        };
        self.dual.focused = !self.dual.focused;
        window.focus(&other.focus_handle(cx), cx);
        cx.notify();
    }

    fn send_to_other(&mut self, mode: ClipboardMode, cx: &mut Context<Self>) {
        let Some(other) = self.other_pane() else {
            return;
        };
        let Some(destination) = other.read(cx).directory() else {
            return self.show_toast(t!("op.cannot_paste_here").to_string(), false, cx);
        };
        let sources = self.active_pane().read(cx).target_paths();
        if sources.is_empty() {
            return;
        }
        self.start_transfer(
            TransferRequest {
                sources,
                destination,
                mode,
                policy: CollisionPolicy::KeepBoth,
                in_place: false,
            },
            cx,
        );
    }

    pub(super) fn with_dual_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(cx.listener(|this, _: &ShowDual, _, cx| this.set_view(ViewMode::Dual, cx)))
            .on_action(cx.listener(|this, _: &SwitchPane, window, cx| this.switch_pane(window, cx)))
            .on_action(cx.listener(|this, _: &CopyToOtherPane, _, cx| {
                this.send_to_other(ClipboardMode::Copy, cx)
            }))
            .on_action(cx.listener(|this, _: &MoveToOtherPane, _, cx| {
                this.send_to_other(ClipboardMode::Cut, cx)
            }))
    }
}
