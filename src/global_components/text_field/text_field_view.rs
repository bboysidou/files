use std::ops::Range;

use gpui::{
    App, Bounds, Context, CursorStyle, EventEmitter, FocusHandle, Focusable, IntoElement,
    MouseButton, MouseDownEvent, Pixels, Render, ShapedLine, SharedString, Window, div, prelude::*,
};

use super::text_field_actions::{
    Backspace, Cancel, Copy, Cut, Delete, DeleteWordBackward, MoveLeft, MoveRight, MoveToEnd,
    MoveToStart, Paste, SelectAll, SelectLeft, SelectRight, Submit, TEXT_FIELD_CONTEXT,
};
use super::text_field_element::TextFieldElement;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextFieldEvent {
    Changed,
    Submitted,
    Cancelled,
}

pub struct TextField {
    pub(super) content: String,
    pub(super) placeholder: SharedString,
    pub(super) selected: Range<usize>,
    pub(super) reversed: bool,
    pub(super) marked: Option<Range<usize>>,
    pub(super) last_layout: Option<ShapedLine>,
    pub(super) last_bounds: Option<Bounds<Pixels>>,
    pub(super) focus_handle: FocusHandle,
}

impl EventEmitter<TextFieldEvent> for TextField {}

impl TextField {
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            content: String::new(),
            placeholder: placeholder.into(),
            selected: 0..0,
            reversed: false,
            marked: None,
            last_layout: None,
            last_bounds: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn text(&self) -> &str {
        &self.content
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.content.clear();
        self.selected = 0..0;
        self.reversed = false;
        self.marked = None;
        cx.notify();
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        let offset = self.index_for_x(event.position.x);
        if event.modifiers.shift {
            self.select_to(offset, cx);
        } else {
            self.move_to(offset, cx);
        }
    }
}

impl Render for TextField {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context(TEXT_FIELD_CONTEXT)
            .track_focus(&self.focus_handle)
            .cursor(CursorStyle::IBeam)
            .flex()
            .flex_1()
            .min_w_0()
            .overflow_hidden()
            .on_action(cx.listener(|this, _: &Backspace, _, cx| this.backspace(cx)))
            .on_action(cx.listener(|this, _: &Delete, _, cx| this.delete(cx)))
            .on_action(
                cx.listener(|this, _: &DeleteWordBackward, _, cx| this.delete_word_backward(cx)),
            )
            .on_action(cx.listener(|this, _: &MoveLeft, _, cx| this.move_left(cx)))
            .on_action(cx.listener(|this, _: &MoveRight, _, cx| this.move_right(cx)))
            .on_action(cx.listener(|this, _: &SelectLeft, _, cx| this.select_left(cx)))
            .on_action(cx.listener(|this, _: &SelectRight, _, cx| this.select_right(cx)))
            .on_action(cx.listener(|this, _: &MoveToStart, _, cx| this.move_to(0, cx)))
            .on_action(
                cx.listener(|this, _: &MoveToEnd, _, cx| this.move_to(this.content.len(), cx)),
            )
            .on_action(cx.listener(|this, _: &SelectAll, _, cx| this.select_all(cx)))
            .on_action(cx.listener(|this, _: &Copy, _, cx| this.copy(cx)))
            .on_action(cx.listener(|this, _: &Cut, _, cx| this.cut(cx)))
            .on_action(cx.listener(|this, _: &Paste, _, cx| this.paste_clipboard(cx)))
            .on_action(cx.listener(|_, _: &Submit, _, cx| cx.emit(TextFieldEvent::Submitted)))
            .on_action(cx.listener(|_, _: &Cancel, _, cx| cx.emit(TextFieldEvent::Cancelled)))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .child(TextFieldElement { field: cx.entity() })
    }
}

impl Focusable for TextField {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
