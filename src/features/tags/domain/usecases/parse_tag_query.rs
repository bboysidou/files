use crate::core::enums::tags::tag_color::TagColor;

const TAG_PREFIX: &str = "tag:";

pub fn parse_tag_query(query: &str) -> (Option<TagColor>, String) {
    let mut tag = None;
    let rest: Vec<&str> = query
        .split_whitespace()
        .filter(|word| {
            let parsed = word.strip_prefix(TAG_PREFIX).and_then(TagColor::from_name);
            if parsed.is_some() {
                tag = parsed;
            }
            parsed.is_none()
        })
        .collect();
    (tag, rest.join(" "))
}

pub fn tag_query(tag: TagColor) -> String {
    format!("{TAG_PREFIX}{}", tag.name().to_lowercase())
}
