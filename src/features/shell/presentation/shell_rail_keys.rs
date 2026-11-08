use gpui::{Context, Div, Focusable, InteractiveElement, Window};

use crate::core::enums::tags::tag_color::TagColor;
use crate::features::places::domain::entities::place::PlaceSection;
use crate::features::shell::presentation::actions::{
    FocusRail, RailLeave, RailNext, RailOpen, RailPrevious,
};
use crate::features::shell::presentation::shell_view::ShellView;
use crate::features::shell::presentation::state::rail_stop::RailStop;

impl ShellView {
    pub(super) fn with_rail_actions(element: Div, cx: &mut Context<Self>) -> Div {
        element
            .on_action(
                cx.listener(|this, _: &FocusRail, window, cx| this.toggle_rail_focus(window, cx)),
            )
            .on_action(cx.listener(|this, _: &RailNext, _, cx| this.step_rail(1, cx)))
            .on_action(cx.listener(|this, _: &RailPrevious, _, cx| this.step_rail(-1, cx)))
            .on_action(
                cx.listener(|this, _: &RailOpen, window, cx| this.open_rail_stop(window, cx)),
            )
            .on_action(cx.listener(|this, _: &RailLeave, window, cx| this.leave_rail(window, cx)))
    }

    fn rail_order(&self) -> Vec<RailStop> {
        let mut order: Vec<RailStop> = Vec::new();
        for section in PlaceSection::ALL {
            order.extend(
                self.places
                    .iter()
                    .enumerate()
                    .filter(|(_, place)| place.section == section)
                    .map(|(index, _)| RailStop::Place(index)),
            );
            if section == PlaceSection::Devices {
                order.extend((0..self.devices.len()).map(RailStop::Device));
            }
        }
        order.extend((0..TagColor::ALL.len()).map(RailStop::Tag));
        order
    }

    fn starting_stop(&self, cx: &Context<Self>) -> RailStop {
        let location = self.browser.read(cx).location(cx);
        if let Some(index) = self
            .places
            .iter()
            .position(|place| place.target == location)
        {
            return RailStop::Place(index);
        }
        if let Some(index) = self
            .device_items(&location)
            .iter()
            .position(|item| item.active)
        {
            return RailStop::Device(index);
        }
        self.rail_order()
            .first()
            .copied()
            .unwrap_or(RailStop::Place(0))
    }

    fn toggle_rail_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.rail_keys.focus.is_focused(window) {
            return self.leave_rail(window, cx);
        }
        self.rail_keys.cursor = self.starting_stop(cx);
        window.focus(&self.rail_keys.focus, cx);
        cx.notify();
    }

    fn step_rail(&mut self, delta: isize, cx: &mut Context<Self>) {
        let order = self.rail_order();
        let Some(last) = order.len().checked_sub(1) else {
            return;
        };
        let position = order
            .iter()
            .position(|stop| *stop == self.rail_keys.cursor)
            .unwrap_or_default();
        let next = (position as isize + delta).clamp(0, last as isize) as usize;
        self.rail_keys.cursor = order[next];
        cx.notify();
    }

    fn open_rail_stop(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.rail_keys.cursor {
            RailStop::Place(index) => self.select_place(index, cx),
            RailStop::Device(index) => self.open_device(index, cx),
            RailStop::Tag(index) => {
                let Some(tag) = TagColor::ALL.get(index).copied() else {
                    return;
                };
                return self
                    .browser
                    .update(cx, |browser, cx| browser.search_tag(tag, window, cx));
            }
        }
        self.leave_rail(window, cx);
    }

    fn leave_rail(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let browser = self.browser.read(cx).focus_handle(cx);
        window.focus(&browser, cx);
        cx.notify();
    }
}
