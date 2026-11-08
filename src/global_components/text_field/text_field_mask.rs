use gpui::Context;

use super::text_field_view::TextField;

const MASK_GLYPH: char = '•';

impl TextField {
    pub fn set_masked(&mut self, masked: bool, cx: &mut Context<Self>) {
        self.masked = masked;
        cx.notify();
    }

    pub(super) fn shown_text(&self) -> String {
        if !self.masked {
            return self.content.clone();
        }
        mask_text(self.content.chars().count())
    }

    pub(super) fn shown_offset(&self, offset: usize) -> usize {
        if !self.masked {
            return offset;
        }
        self.content[..offset].chars().count() * MASK_GLYPH.len_utf8()
    }

    pub(super) fn content_offset(&self, shown: usize) -> usize {
        if !self.masked {
            return shown;
        }
        let chars = shown / MASK_GLYPH.len_utf8();
        self.content
            .char_indices()
            .nth(chars)
            .map_or(self.content.len(), |(offset, _)| offset)
    }
}

fn mask_text(count: usize) -> String {
    std::iter::repeat_n(MASK_GLYPH, count).collect()
}
