mod core;
mod features;
mod global_components;

use gpui::{App, AppContext, Bounds, Focusable, KeyBinding, WindowBounds, WindowOptions, px, size};

use crate::core::assets::embedded_assets::EmbeddedAssets;
use crate::core::constants::app::{APP_ID, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::core::constants::keymap::TOGGLE_RAIL_KEYS;
use crate::core::helpers::i18n::system_locale::system_locale;
use crate::features::places::data::repositories::xdg_places_repository::XdgPlacesRepository;
use crate::features::shell::presentation::actions::ToggleRail;
use crate::features::shell::presentation::shell_view::ShellView;

rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    rust_i18n::set_locale(&system_locale());
    gpui_platform::application()
        .with_assets(EmbeddedAssets)
        .run(|cx: &mut App| {
            cx.bind_keys([KeyBinding::new(TOGGLE_RAIL_KEYS, ToggleRail, None)]);
            let bounds = Bounds::centered(None, size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)), cx);
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                app_id: Some(APP_ID.into()),
                ..Default::default()
            };
            cx.open_window(options, |window, cx| {
                let view = cx.new(|cx| ShellView::new(&XdgPlacesRepository, cx));
                window.focus(&view.focus_handle(cx), cx);
                view
            })
            .expect("failed to open main window");
            cx.activate(true);
        });
}
