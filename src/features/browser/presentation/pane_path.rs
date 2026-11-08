use std::path::{Path, PathBuf};

use gpui::{Context, Focusable, Window};
use rust_i18n::t;

use crate::core::constants::network::SFTP_PREFIX;
use crate::core::constants::paths::HOME_SHORTHAND;
use crate::core::enums::places::crumb_root::CrumbRoot;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::pane_event::PaneEvent;
use crate::features::network::domain::usecases::network_uri::network_uri;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::global_components::text_field::text_field_view::TextFieldEvent;

impl PaneView {
    pub(super) fn start_path_entry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let trail = self.trail();
        let text = match (&self.location, trail.root) {
            (PlaceTarget::Directory(path), CrumbRoot::Network) => {
                path.to_string_lossy().into_owned()
            }
            _ => trail.display_path(),
        };
        let length = text.len();
        self.path_field
            .update(cx, |field, cx| field.set_text(&text, 0..length, cx));
        self.path_editing = true;
        window.focus(&self.path_field.focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn start_connect_entry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = SFTP_PREFIX;
        self.path_field.update(cx, |field, cx| {
            field.set_text(text, text.len()..text.len(), cx)
        });
        self.path_editing = true;
        window.focus(&self.path_field.focus_handle(cx), cx);
        cx.notify();
    }

    pub(super) fn on_path_event(
        &mut self,
        event: TextFieldEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            TextFieldEvent::Changed => {}
            TextFieldEvent::Submitted => self.commit_path(window, cx),
            TextFieldEvent::Cancelled => self.stop_path_entry(window, cx),
        }
    }

    fn resolve_typed(&self, typed: &str) -> Option<PathBuf> {
        let home = self.setup.home.as_deref();
        let expanded = match typed.strip_prefix(HOME_SHORTHAND) {
            Some(rest) => home?.join(rest.trim_start_matches('/')),
            None => PathBuf::from(typed),
        };
        if expanded.is_absolute() {
            Some(expanded)
        } else {
            self.directory().map(|directory| directory.join(&expanded))
        }
    }

    fn commit_path(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let typed = self.path_field.read(cx).text().trim().to_string();
        self.stop_path_entry(window, cx);
        if typed.is_empty() {
            return;
        }
        if let Some(uri) = network_uri(&typed) {
            return self.connect_network(uri, None, cx);
        }
        match self.resolve_typed(&typed).filter(|path| Path::is_dir(path)) {
            Some(path) => self.navigate(PlaceTarget::Directory(path), cx),
            None => cx.emit(PaneEvent::Message(
                t!("path.not_found", path = typed).to_string(),
            )),
        }
    }

    fn stop_path_entry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.path_editing = false;
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }
}
