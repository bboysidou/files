use gpui::{App, IntoElement, RenderOnce, Window, div, prelude::*, px, rgb};
use rust_i18n::t;

use crate::core::constants::files::PERMISSION_TOP_BIT;
use crate::core::theme::colors::FG_MUTED;
use crate::core::theme::metrics::{PERMISSION_CELL_WIDTH, PERMISSION_LABEL_WIDTH, TOOLBAR_GAP};
use crate::global_components::handlers::{ClickHandler, IndexHandler};
use crate::global_components::toggle_switch::ToggleSwitch;

const CLASSES: [&str; 3] = ["props.owner_class", "props.group", "props.others"];
const RIGHTS: [&str; 3] = ["props.read", "props.write", "props.execute"];

#[derive(IntoElement)]
pub struct PermissionGrid {
    pub mode: u32,
    pub on_toggle: IndexHandler,
}

impl RenderOnce for PermissionGrid {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let header = div()
            .flex()
            .text_color(rgb(FG_MUTED))
            .child(div().w(px(PERMISSION_LABEL_WIDTH)))
            .children(RIGHTS.map(|key| {
                div()
                    .w(px(PERMISSION_CELL_WIDTH))
                    .child(t!(key).to_string())
            }));
        let (mode, on_toggle) = (self.mode, self.on_toggle);
        let rows = CLASSES.iter().enumerate().map(move |(row, key)| {
            let cells = (0..RIGHTS.len()).map(|column| {
                let index = row * RIGHTS.len() + column;
                let toggle = on_toggle.clone();
                let handler: ClickHandler =
                    std::rc::Rc::new(move |window, cx| toggle(index, window, cx));
                div().w(px(PERMISSION_CELL_WIDTH)).child(ToggleSwitch {
                    id: ("perm", index).into(),
                    on: mode & (PERMISSION_TOP_BIT >> index) != 0,
                    on_toggle: handler,
                })
            });
            div()
                .flex()
                .items_center()
                .child(
                    div()
                        .w(px(PERMISSION_LABEL_WIDTH))
                        .child(t!(*key).to_string()),
                )
                .children(cells)
        });
        div()
            .flex()
            .flex_col()
            .gap(px(TOOLBAR_GAP))
            .child(header)
            .children(rows)
    }
}
