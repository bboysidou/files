use std::ops::Range;

use gpui::{Bounds, Context, EntityInputHandler, Pixels, Point, UTF16Selection, Window, point};

use super::text_field_view::{TextField, TextFieldEvent};
use crate::core::helpers::text::utf16_offset::{from_utf16_offset, to_utf16_offset};

impl TextField {
    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        to_utf16_offset(&self.content, range.start)..to_utf16_offset(&self.content, range.end)
    }

    fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        from_utf16_offset(&self.content, range.start)..from_utf16_offset(&self.content, range.end)
    }

    fn target_range(&self, range_utf16: Option<&Range<usize>>) -> Range<usize> {
        range_utf16
            .map(|range| self.range_from_utf16(range))
            .or_else(|| self.marked.clone())
            .unwrap_or_else(|| self.selected.clone())
    }

    fn layout_matches(&self) -> bool {
        self.last_layout
            .as_ref()
            .is_some_and(|layout| layout.text.as_ref() == self.content)
    }
}

impl EntityInputHandler for TextField {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        adjusted_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected),
            reversed: self.reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked.as_ref().map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = self.target_range(range_utf16.as_ref());
        self.replace(range, text, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = self.target_range(range_utf16.as_ref());
        self.content.replace_range(range.clone(), new_text);
        let inserted_end = range.start + new_text.len();
        self.marked = (!new_text.is_empty()).then_some(range.start..inserted_end);
        self.selected = new_selected_range_utf16
            .map(|selected| {
                let inserted = &self.content[range.start..inserted_end];
                range.start + from_utf16_offset(inserted, selected.start)
                    ..range.start + from_utf16_offset(inserted, selected.end)
            })
            .unwrap_or(inserted_end..inserted_end);
        self.reversed = false;
        cx.emit(TextFieldEvent::Changed);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        if !self.layout_matches() {
            return None;
        }
        let layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        Some(Bounds::from_corners(
            point(
                bounds.left() + layout.x_for_index(range.start),
                bounds.top(),
            ),
            point(
                bounds.left() + layout.x_for_index(range.end),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        position: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        if !self.layout_matches() {
            return None;
        }
        let local = self.last_bounds?.localize(&position)?;
        let index = self.last_layout.as_ref()?.closest_index_for_x(local.x);
        Some(to_utf16_offset(&self.content, index))
    }
}
