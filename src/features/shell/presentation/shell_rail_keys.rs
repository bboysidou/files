use gpui::{Context, Div, Focusable, InteractiveElement, Window};

use crate::features::places::domain::entities::place::PlaceSection;
use crate::features::shell::presentation::actions::{
    FocusRail, RailLeave, RailNext, RailOpen, RailPrevious,
};
use crate::features::shell::presentation::shell_view::ShellView;

impl ShellView {
    pub(super) fn with_rail_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(
                cx.listener(|this, _: &FocusRail, window, cx| this.toggle_rail_focus(window, cx)),
            )
            .on_action(cx.listener(|this, _: &RailNext, _, cx| this.step_rail(1, cx)))
            .on_action(cx.listener(|this, _: &RailPrevious, _, cx| this.step_rail(-1, cx)))
            .on_action(
                cx.listener(|this, _: &RailOpen, window, cx| this.open_rail_place(window, cx)),
            )
            .on_action(cx.listener(|this, _: &RailLeave, window, cx| this.leave_rail(window, cx)))
    }

    pub(super) fn rail_order(&self) -> Vec<usize> {
        PlaceSection::ALL
            .iter()
            .flat_map(|section| {
                self.places
                    .iter()
                    .enumerate()
                    .filter(move |(_, place)| place.section == *section)
                    .map(|(index, _)| index)
            })
            .collect()
    }

    fn toggle_rail_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.rail_keys.focus.is_focused(window) {
            return self.leave_rail(window, cx);
        }
        let location = self.browser.read(cx).location(cx);
        self.rail_keys.cursor = self
            .places
            .iter()
            .position(|place| place.target == location)
            .unwrap_or_else(|| self.rail_order().first().copied().unwrap_or_default());
        window.focus(&self.rail_keys.focus, cx);
        cx.notify();
    }

    fn step_rail(&mut self, delta: isize, cx: &mut Context<Self>) {
        let order = self.rail_order();
        let Some(position) = order
            .iter()
            .position(|index| *index == self.rail_keys.cursor)
        else {
            return;
        };
        let next = (position as isize + delta).clamp(0, order.len() as isize - 1) as usize;
        self.rail_keys.cursor = order[next];
        cx.notify();
    }

    fn open_rail_place(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.select_place(self.rail_keys.cursor, cx);
        self.leave_rail(window, cx);
    }

    fn leave_rail(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let browser = self.browser.read(cx).focus_handle(cx);
        window.focus(&browser, cx);
        cx.notify();
    }
}
