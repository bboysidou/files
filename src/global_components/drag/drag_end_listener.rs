use gpui::{
    App, DispatchPhase, FileDropEvent, IntoElement, MouseUpEvent, RenderOnce, Styled, Window,
    canvas,
};

use crate::global_components::handlers::ClickHandler;

#[derive(IntoElement)]
pub struct DragEndListener {
    pub on_end: ClickHandler,
    pub on_release: ClickHandler,
}

impl RenderOnce for DragEndListener {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (on_end, on_release) = (self.on_end, self.on_release);
        canvas(
            |_, _, _| (),
            move |_, _, window, _| {
                let (on_end, on_release) = (on_end.clone(), on_release.clone());
                window.on_mouse_event(move |event: &FileDropEvent, phase, window, cx| {
                    if phase == DispatchPhase::Bubble && matches!(event, FileDropEvent::Ended) {
                        on_end(window, cx);
                    }
                });
                window.on_mouse_event(move |_: &MouseUpEvent, phase, window, cx| {
                    if phase == DispatchPhase::Bubble {
                        on_release(window, cx);
                    }
                });
            },
        )
        .absolute()
        .size_0()
    }
}
