pub fn to_utf16_offset(text: &str, offset: usize) -> usize {
    text[..offset.min(text.len())].encode_utf16().count()
}

pub fn from_utf16_offset(text: &str, offset_utf16: usize) -> usize {
    let mut units = 0;
    for (index, character) in text.char_indices() {
        if units >= offset_utf16 {
            return index;
        }
        units += character.len_utf16();
    }
    text.len()
}
