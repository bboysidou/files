use std::time::{Duration, Instant};

use gpui::{Context, Div, InteractiveElement, KeyDownEvent, Window};

use crate::core::constants::files::TYPE_AHEAD_RESET_MS;
use crate::core::enums::browser::key_preset::KeyPreset;
use crate::features::browser::presentation::pane_view::PaneView;
use crate::features::browser::presentation::state::active_key_preset::ActiveKeyPreset;

const SPACE: &str = " ";

impl PaneView {
    pub(super) fn with_type_ahead(element: Div, cx: &mut Context<Self>) -> Div {
        element.on_key_down(
            cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.type_ahead(event, window, cx)
            }),
        )
    }

    fn type_ahead(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let classic = cx
            .try_global::<ActiveKeyPreset>()
            .is_some_and(|active| active.0 == KeyPreset::Classic);
        if !classic || !self.focus_handle.is_focused(window) {
            return;
        }
        let modifiers = &event.keystroke.modifiers;
        if modifiers.control || modifiers.alt || modifiers.platform || modifiers.function {
            return;
        }
        let Some(text) = event.keystroke.key_char.as_deref() else {
            return;
        };
        if text == SPACE || text.chars().any(char::is_control) {
            return;
        }
        let now = Instant::now();
        let expired = self
            .type_ahead
            .at
            .is_none_or(|at| now.duration_since(at) > Duration::from_millis(TYPE_AHEAD_RESET_MS));
        if expired {
            self.type_ahead.typed.clear();
        }
        self.type_ahead.typed.push_str(text);
        self.type_ahead.at = Some(now);
        let needle = self.type_ahead.typed.to_lowercase();
        if let Some(index) = self
            .entries
            .iter()
            .position(|entry| entry.name.to_lowercase().starts_with(&needle))
        {
            self.selection.move_to(index);
            self.reveal(cx);
        }
        cx.stop_propagation();
    }
}
