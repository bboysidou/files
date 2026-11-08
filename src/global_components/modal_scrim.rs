use gpui::{
    AnyElement, App, FocusHandle, IntoElement, MouseButton, Pixels, RenderOnce, Size, Window, div,
    prelude::*, rgba,
};

use crate::core::theme::colors::SCRIM;
use crate::global_components::handlers::ClickHandler;

#[derive(IntoElement)]
pub struct ModalScrim {
    pub focus: FocusHandle,
    pub context: &'static str,
    pub viewport: Size<Pixels>,
    pub on_dismiss: ClickHandler,
    pub panel: AnyElement,
}

impl RenderOnce for ModalScrim {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_dismiss = self.on_dismiss;
        div()
            .track_focus(&self.focus)
            .key_context(self.context)
            .w(self.viewport.width)
            .h(self.viewport.height)
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(SCRIM))
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                on_dismiss(window, cx)
            })
            .child(self.panel)
    }
}
