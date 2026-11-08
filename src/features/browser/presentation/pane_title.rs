use gpui::{App, SharedString};
use rust_i18n::t;

use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::server_names::ServerNames;
use crate::features::network::domain::usecases::network_mount_root::network_mount_root;
use crate::features::network::domain::usecases::saved_server_name::saved_server_name;
use crate::features::places::domain::entities::crumb_trail::CrumbTrail;
use crate::features::places::domain::entities::place::PlaceTarget;

impl PaneView {
    pub(super) fn title(&self, cx: &App) -> SharedString {
        if self.search.is_some() {
            return t!("search.title").to_string().into();
        }
        if let Some(place) = self
            .setup
            .places
            .iter()
            .find(|place| place.target == self.location)
        {
            return t!(place.kind.label_key()).to_string().into();
        }
        match &self.location {
            PlaceTarget::Directory(path) => match network_mount_root(path) {
                Some((mount, label)) if mount == *path => named(label, cx),
                _ => path.file_name().map_or_else(
                    || self.trail(cx).display_path(),
                    |name| name.to_string_lossy().into_owned(),
                ),
            },
            _ => self.trail(cx).display_path(),
        }
        .into()
    }

    pub(super) fn trail(&self, cx: &App) -> CrumbTrail {
        let mut trail = CrumbTrail::for_target(&self.location, self.setup.home.as_deref());
        trail.root_label = trail.root_label.take().map(|label| named(label, cx));
        trail
    }
}

fn named(label: String, cx: &App) -> String {
    cx.try_global::<ServerNames>()
        .and_then(|names| saved_server_name(&label, &names.0))
        .unwrap_or(label)
}
