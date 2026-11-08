use gpui::{
    AnyElement, ClickEvent, Context, DragMoveEvent, EmptyView, Entity, IntoElement, div,
    prelude::*, px, relative,
};

use crate::core::theme::metrics::{DUAL_GAP, DUAL_SPLIT_DEFAULT, DUAL_SPLIT_MAX, DUAL_SPLIT_MIN};
use crate::features::browser::presentation::browser_view::BrowserView;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::divider_drag::DividerDrag;

const DOUBLE_CLICK: usize = 2;

impl BrowserView {
    pub(super) fn dual_split(
        &self,
        panes: [(Entity<PaneView>, bool); 2],
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let [(main, main_focused), (partner, partner_focused)] = panes;
        let divider = div()
            .id("dual-divider")
            .flex_shrink_0()
            .w(px(DUAL_GAP))
            .h_full()
            .cursor_col_resize()
            .on_click(cx.listener(|this, event: &ClickEvent, _, cx| {
                if event.click_count() == DOUBLE_CLICK {
                    this.set_split(DUAL_SPLIT_DEFAULT, cx);
                }
            }))
            .on_drag(DividerDrag, |_, _, _, cx| cx.new(|_| EmptyView));
        div()
            .size_full()
            .flex()
            .on_drag_move(
                cx.listener(|this, event: &DragMoveEvent<DividerDrag>, _, cx| {
                    let bounds = event.bounds;
                    let width = bounds.size.width / px(1.0);
                    if width <= 0.0 {
                        return;
                    }
                    let offset = (event.event.position.x - bounds.left()) / px(1.0);
                    this.set_split(offset / width, cx);
                }),
            )
            .child(
                Self::dual_frame(main, main_focused)
                    .flex_none()
                    .w(relative(self.dual.split)),
            )
            .child(divider)
            .child(Self::dual_frame(partner, partner_focused))
            .into_any_element()
    }

    fn set_split(&mut self, split: f32, cx: &mut Context<Self>) {
        let split = split.clamp(DUAL_SPLIT_MIN, DUAL_SPLIT_MAX);
        if split != self.dual.split {
            self.dual.split = split;
            cx.notify();
        }
    }
}
