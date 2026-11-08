use std::{env, process};

mod core;
mod features;
mod global_components;

use gpui::App;

use crate::core::assets::embedded_assets::EmbeddedAssets;
use crate::core::assets::embedded_fonts::embedded_fonts;
use crate::core::dependency_injection::integration::{default_handler, instance_bridge};
use crate::core::dependency_injection::picker::portal_registration;
use crate::core::dependency_injection::settings::settings_repository;
use crate::core::helpers::i18n::supported_locale::supported_locale;
use crate::core::helpers::i18n::system_locale::system_locale;
use crate::features::integration::domain::entities::launch_request::LaunchRequest;
use crate::features::integration::domain::usecases::parse_launch::parse_launch;
use crate::features::picker::presentation::portal_host::serve_portal;
use crate::features::shell::presentation::app_key_bindings::bind_app_keys;
use crate::features::shell::presentation::shell_windows::open_shell_window;
use crate::features::shell::presentation::state::shell_start::{ShellStart, WindowRole};

rust_i18n::i18n!("locales", fallback = "en");

fn report(result: Result<String, String>) {
    match result {
        Ok(message) => println!("{message}"),
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    }
}

fn run_default(enable: bool) {
    let handler = default_handler();
    report(if enable {
        handler.enable()
    } else {
        handler.disable()
    });
}

fn bind_keys(cx: &mut App) {
    let _ = cx.text_system().add_fonts(embedded_fonts());
    bind_app_keys(settings_repository().load().key_preset, cx);
}

fn main() {
    let locales = rust_i18n::available_locales!();
    let available: Vec<&str> = locales.iter().map(|locale| locale.as_ref()).collect();
    rust_i18n::set_locale(&supported_locale(&system_locale(), &available));
    let arguments: Vec<String> = env::args().skip(1).collect();
    let working = env::current_dir().unwrap_or_default();
    let launch = parse_launch(&arguments, &working);
    match launch {
        LaunchRequest::SetDefault(enable) => return run_default(enable),
        LaunchRequest::PortalSetup => return report(portal_registration().register()),
        LaunchRequest::Portal => {}
        _ if instance_bridge().forward(&launch) => return,
        _ => {}
    }
    gpui_platform::application()
        .with_assets(EmbeddedAssets)
        .run(move |cx: &mut App| {
            bind_keys(cx);
            if launch == LaunchRequest::Portal {
                serve_portal(cx);
            } else {
                open_shell_window(
                    ShellStart {
                        launch,
                        role: WindowRole::Primary,
                    },
                    cx,
                );
            }
        });
}
