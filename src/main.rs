use gpui::{
    App, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};

const APP_ID: &str = "files";
const FONT_FAMILY: &str = "JetBrains Mono";
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 800.0;
const FRAME_WIDTH: f32 = 8.0;
const SHEET_RADIUS: f32 = 20.0;
const FRAME_COLOR: u32 = 0x000000;
const SHEET_COLOR: u32 = 0x0A0A0A;
const TEXT_COLOR: u32 = 0xE5E5E5;

struct Shell;

impl Render for Shell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(FRAME_COLOR))
            .p(px(FRAME_WIDTH))
            .font_family(FONT_FAMILY)
            .text_color(rgb(TEXT_COLOR))
            .child(
                div()
                    .size_full()
                    .bg(rgb(SHEET_COLOR))
                    .rounded(px(SHEET_RADIUS))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(APP_ID),
            )
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                app_id: Some(APP_ID.into()),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Shell),
        )
        .expect("failed to open main window");
        cx.activate(true);
    });
}
