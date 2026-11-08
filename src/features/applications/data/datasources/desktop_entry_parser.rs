use std::collections::HashMap;

use crate::core::constants::applications::{
    APPLICATION_TYPE, COMMENT_PREFIX, DESKTOP_ENTRY_GROUP, ESCAPE_MARK, EXEC_KEY, GROUP_PREFIX,
    HIDDEN_KEY, KEY_SEPARATOR, LIST_SEPARATOR, MIME_TYPE_KEY, NAME_KEY, TERMINAL_KEY, TRUE_VALUE,
    TYPE_KEY,
};
use crate::features::applications::data::models::desktop_entry::DesktopEntry;
use crate::features::applications::domain::entities::desktop_app::DesktopApp;

pub fn parse_desktop_entry(id: String, text: &str) -> Option<DesktopEntry> {
    let fields = entry_fields(text);
    let flag = |key: &str| fields.get(key).is_some_and(|value| *value == TRUE_VALUE);
    if fields.get(TYPE_KEY) != Some(&APPLICATION_TYPE) || flag(HIDDEN_KEY) {
        return None;
    }
    let app = DesktopApp {
        id,
        name: unescape(fields.get(NAME_KEY)?),
        exec: unescape(fields.get(EXEC_KEY)?),
        terminal: flag(TERMINAL_KEY),
    };
    let mime_types = fields
        .get(MIME_TYPE_KEY)
        .map(|value| {
            value
                .split(LIST_SEPARATOR)
                .filter(|kind| !kind.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    Some(DesktopEntry { app, mime_types })
}

fn entry_fields(text: &str) -> HashMap<&str, &str> {
    let mut fields = HashMap::new();
    let mut in_entry = false;
    for line in text.lines().map(str::trim) {
        if line.starts_with(GROUP_PREFIX) {
            in_entry = line == DESKTOP_ENTRY_GROUP;
        } else if in_entry
            && !line.starts_with(COMMENT_PREFIX)
            && let Some((key, value)) = line.split_once(KEY_SEPARATOR)
        {
            fields.entry(key.trim()).or_insert(value.trim());
        }
    }
    fields
}

fn unescape(value: &str) -> String {
    let mut plain = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character != ESCAPE_MARK {
            plain.push(character);
            continue;
        }
        match chars.next() {
            Some('s') => plain.push(' '),
            Some('n') => plain.push('\n'),
            Some('t') => plain.push('\t'),
            Some('r') => plain.push('\r'),
            Some(ESCAPE_MARK) => plain.push(ESCAPE_MARK),
            Some(other) => {
                plain.push(ESCAPE_MARK);
                plain.push(other);
            }
            None => plain.push(ESCAPE_MARK),
        }
    }
    plain
}
