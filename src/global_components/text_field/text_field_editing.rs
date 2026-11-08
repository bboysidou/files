use std::ops::Range;

use gpui::{ClipboardItem, Context, Pixels};

use super::text_field_view::{TextField, TextFieldEvent};

const LINE_BREAKS: [char; 2] = ['\n', '\r'];

impl TextField {
    pub(super) fn cursor(&self) -> usize {
        if self.reversed {
            self.selected.start
        } else {
            self.selected.end
        }
    }

    pub(super) fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected = offset..offset;
        self.reversed = false;
        cx.notify();
    }

    pub(super) fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.reversed {
            self.selected.start = offset;
        } else {
            self.selected.end = offset;
        }
        if self.selected.end < self.selected.start {
            self.reversed = !self.reversed;
            self.selected = self.selected.end..self.selected.start;
        }
        cx.notify();
    }

    pub(super) fn replace(&mut self, range: Range<usize>, text: &str, cx: &mut Context<Self>) {
        self.content.replace_range(range.clone(), text);
        let end = range.start + text.len();
        self.selected = end..end;
        self.reversed = false;
        self.marked = None;
        cx.emit(TextFieldEvent::Changed);
        cx.notify();
    }

    pub(super) fn index_for_x(&self, x: Pixels) -> usize {
        match (&self.last_layout, &self.last_bounds) {
            (Some(layout), Some(bounds)) => layout.closest_index_for_x(x - bounds.left()),
            _ => self.content.len(),
        }
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content[..offset]
            .char_indices()
            .next_back()
            .map_or(0, |(index, _)| index)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content[offset..]
            .chars()
            .next()
            .map_or(offset, |character| offset + character.len_utf8())
    }

    fn previous_word_start(&self, offset: usize) -> usize {
        let trimmed = self.content[..offset].trim_end();
        trimmed
            .char_indices()
            .rfind(|(_, character)| !character.is_alphanumeric())
            .map_or(0, |(index, character)| index + character.len_utf8())
    }

    fn delete_towards(&mut self, boundary: usize, cx: &mut Context<Self>) {
        if self.selected.is_empty() {
            self.select_to(boundary, cx);
        }
        self.replace(self.selected.clone(), "", cx);
    }

    pub(super) fn backspace(&mut self, cx: &mut Context<Self>) {
        self.delete_towards(self.previous_boundary(self.cursor()), cx);
    }

    pub(super) fn delete(&mut self, cx: &mut Context<Self>) {
        self.delete_towards(self.next_boundary(self.cursor()), cx);
    }

    pub(super) fn delete_word_backward(&mut self, cx: &mut Context<Self>) {
        self.delete_towards(self.previous_word_start(self.cursor()), cx);
    }

    pub(super) fn move_left(&mut self, cx: &mut Context<Self>) {
        let target = if self.selected.is_empty() {
            self.previous_boundary(self.cursor())
        } else {
            self.selected.start
        };
        self.move_to(target, cx);
    }

    pub(super) fn move_right(&mut self, cx: &mut Context<Self>) {
        let target = if self.selected.is_empty() {
            self.next_boundary(self.cursor())
        } else {
            self.selected.end
        };
        self.move_to(target, cx);
    }

    pub(super) fn select_left(&mut self, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor()), cx);
    }

    pub(super) fn select_right(&mut self, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor()), cx);
    }

    pub(super) fn select_all(&mut self, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx);
    }

    pub(super) fn copy(&mut self, cx: &mut Context<Self>) {
        if !self.selected.is_empty() {
            let text = self.content[self.selected.clone()].to_string();
            cx.write_to_clipboard(ClipboardItem::new_string(text));
        }
    }

    pub(super) fn cut(&mut self, cx: &mut Context<Self>) {
        self.copy(cx);
        if !self.selected.is_empty() {
            self.replace(self.selected.clone(), "", cx);
        }
    }

    pub(super) fn paste_clipboard(&mut self, cx: &mut Context<Self>) {
        let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) else {
            return;
        };
        let single_line = text.replace(LINE_BREAKS, " ");
        self.replace(self.selected.clone(), &single_line, cx);
    }
}
