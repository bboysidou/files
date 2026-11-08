use std::{env, process};

mod core;
mod features;
mod global_components;

use gpui::{App, AppContext, Bounds, Focusable, KeyBinding, WindowBounds, WindowOptions, px, size};

use crate::core::assets::embedded_assets::EmbeddedAssets;
use crate::core::assets::embedded_fonts::embedded_fonts;
use crate::core::constants::app::{APP_ID, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::core::constants::keymap::TOGGLE_RAIL_KEYS;
use crate::core::dependency_injection::integration::{default_handler, instance_bridge};
use crate::core::dependency_injection::places::places_repository;
use crate::core::helpers::i18n::system_locale::system_locale;
use crate::features::browser::presentation::key_bindings::browser_key_bindings;
use crate::features::integration::domain::entities::launch_request::LaunchRequest;
use crate::features::integration::domain::usecases::parse_launch::parse_launch;
use crate::features::shell::presentation::actions::ToggleRail;
use crate::features::shell::presentation::shell_view::ShellView;
use crate::global_components::text_field::text_field_actions::text_field_key_bindings;

rust_i18n::i18n!("locales", fallback = "en");

fn run_default(enable: bool) {
    let handler = default_handler();
    let result = if enable {
        handler.enable()
    } else {
        handler.disable()
    };
    match result {
        Ok(message) => println!("{message}"),
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    }
}

fn main() {
    rust_i18n::set_locale(&system_locale());
    let arguments: Vec<String> = env::args().skip(1).collect();
    let working = env::current_dir().unwrap_or_default();
    let launch = parse_launch(&arguments, &working);
    if let LaunchRequest::SetDefault(enable) = launch {
        return run_default(enable);
    }
    if instance_bridge().forward(&launch) {
        return;
    }
    gpui_platform::application()
        .with_assets(EmbeddedAssets)
        .run(move |cx: &mut App| {
            let _ = cx.text_system().add_fonts(embedded_fonts());
            cx.bind_keys([KeyBinding::new(TOGGLE_RAIL_KEYS, ToggleRail, None)]);
            cx.bind_keys(browser_key_bindings());
            cx.bind_keys(text_field_key_bindings());
            let bounds = Bounds::centered(None, size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)), cx);
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                app_id: Some(APP_ID.into()),
                ..Default::default()
            };
            cx.open_window(options, |window, cx| {
                let view = cx.new(|cx| {
                    ShellView::new(places_repository().as_ref(), launch.clone(), window, cx)
                });
                window.focus(&view.focus_handle(cx), cx);
                view
            })
            .expect("failed to open main window");
            cx.activate(true);
        });
}
