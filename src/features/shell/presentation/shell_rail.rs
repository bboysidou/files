use std::rc::Rc;

use gpui::{Context, Window};

use crate::features::shell::presentation::components::rail::Rail;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::global_components::handlers::{ClickHandler, PointHandler};

impl ShellView {
    pub(super) fn rail_view(&self, window: &mut Window, cx: &mut Context<Self>) -> Rail {
        let (on_toggle, on_select) = Self::handlers(cx);
        let (on_device, on_eject) = Self::device_handlers(cx);
        let (on_network, on_unmount) = Self::network_handlers(cx);
        let (on_phone, on_phone_eject) = Self::phone_handlers(cx);
        let (on_ssh, on_ssh_eject) = Self::ssh_handlers(cx);
        let (on_cancel_job, on_search, on_tag) = self.browser_handlers();
        let location = self.browser.read(cx).location(cx);
        let active = self
            .places
            .iter()
            .position(|place| place.target == location);
        let rail_cursor = self
            .rail_keys
            .focus
            .is_focused(window)
            .then_some(self.rail_keys.cursor);
        Rail {
            places: self.places.clone(),
            active,
            motion: self.rail,
            on_toggle,
            on_select,
            on_trash_context: self.trash_context_handler(),
            on_drop_place: self.place_drop_handler(),
            on_search,
            on_connect: self.connect_handler(),
            devices: self.device_items(&location),
            on_device,
            on_device_context: self.device_context_handler(cx),
            on_eject,
            phones: self.phone_items(&location),
            on_phone,
            on_phone_eject,
            favorites: self.favorite_rail(&location, cx),
            ssh: self.ssh_items(&location, cx),
            on_ssh,
            on_ssh_eject,
            network: self.network_items(&location),
            on_network,
            on_unmount,
            on_edit_network: Self::network_edit_handler(cx),
            job: self.browser.read(cx).job_card(),
            on_cancel_job,
            on_tag,
            focus: self.rail_keys.focus.clone(),
            cursor: rail_cursor,
        }
    }

    fn trash_context_handler(&self) -> PointHandler {
        let browser = self.browser.downgrade();
        Rc::new(move |position, window, cx| {
            let _ = browser.update(cx, |browser, cx| {
                browser.open_trash_menu(position, window, cx)
            });
        })
    }

    fn connect_handler(&self) -> ClickHandler {
        let browser = self.browser.downgrade();
        Rc::new(move |window, cx| {
            let _ = browser.update(cx, |browser, cx| browser.open_server_edit(None, window, cx));
        })
    }
}
