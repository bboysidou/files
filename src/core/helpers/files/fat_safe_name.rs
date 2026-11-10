use crate::core::constants::files::{
    FAT_FORBIDDEN_CHARS, FAT_REPLACEMENT_CHAR, FAT_TRAILING_CHARS,
};

pub fn fat_safe_name(name: &str) -> String {
    let replaced: String = name
        .chars()
        .map(|character| {
            if character.is_control() || FAT_FORBIDDEN_CHARS.contains(&character) {
                FAT_REPLACEMENT_CHAR
            } else {
                character
            }
        })
        .collect();
    let trimmed = replaced.trim_end_matches(FAT_TRAILING_CHARS);
    if trimmed.is_empty() {
        FAT_REPLACEMENT_CHAR.to_string()
    } else {
        trimmed.to_string()
    }
}
