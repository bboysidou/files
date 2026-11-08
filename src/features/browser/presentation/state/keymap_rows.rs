use crate::core::enums::browser::key_preset::KeyPreset;

pub const KEYMAP_ROWS: [(&str, &str); 43] = [
    ("keys.move", "j k · ↓ ↑"),
    ("keys.parent_open", "h l · ← →"),
    ("keys.open", "Enter"),
    ("keys.open_with", "o"),
    ("keys.properties", "i"),
    ("keys.terminal", "S"),
    ("keys.extract", "e"),
    ("keys.compress", "c"),
    ("keys.browse_archive", "B"),
    ("keys.quick_look", "Space"),
    ("keys.quick_look_inside", "h l · ← → / j k · ↑ ↓"),
    ("keys.first_last", "gg / G"),
    ("keys.select", "v / Shift+↓ ↑"),
    ("keys.select_all", "Ctrl+A"),
    ("keys.invert", "V"),
    ("keys.folder_sizes", "s"),
    ("keys.dismiss", "Esc"),
    ("keys.clipboard", "y x p · Ctrl+C X V"),
    ("keys.copy_path", "Ctrl+Shift+C"),
    ("keys.rename", "r · F2"),
    ("keys.trash", "dd · Delete"),
    ("keys.undo", "z · Ctrl+Z / Ctrl+Shift+Z"),
    ("keys.new_folder", "Ctrl+Shift+N"),
    ("keys.new_file", "N"),
    ("keys.duplicate", "Ctrl+D"),
    ("keys.paste_link", "Ctrl+Shift+V"),
    ("keys.filter", "/"),
    ("keys.search", "f · Ctrl+F"),
    ("keys.path", ": · Ctrl+L"),
    ("keys.views", "Ctrl+1 / Ctrl+3"),
    ("keys.preview", "F3"),
    ("keys.preview_pages", "Shift+J · Shift+K"),
    ("keys.tabs", "t / w · Ctrl+T / Ctrl+W"),
    ("keys.go_tab", "1–9 · [ ]"),
    ("keys.move_tab", "{ } · Ctrl+Shift+PgUp / PgDn"),
    ("keys.hidden", "Ctrl+H"),
    ("keys.tag", "T 1–7 · T 0"),
    ("keys.menu", "m · right click"),
    ("keys.rail_focus", "Ctrl+0, then j k · Enter · Esc"),
    ("keys.history", "Alt+← / Alt+→"),
    ("keys.reload", "F5"),
    ("keys.sheet", "?"),
    ("keys.settings", ", · Ctrl+,"),
];

pub const CLASSIC_KEYS: [(&str, &str); 31] = [
    ("keys.move", "↓ ↑"),
    ("keys.parent_open", "← → · Backspace"),
    ("keys.open_with", "Shift+F10 menu"),
    ("keys.properties", "Alt+Enter"),
    ("keys.terminal", "Shift+F10 menu"),
    ("keys.extract", "Shift+F10 menu"),
    ("keys.compress", "Shift+F10 menu"),
    ("keys.browse_archive", "Shift+F10 menu"),
    ("keys.first_last", "Home / End"),
    ("keys.select", "Ctrl+Space / Shift+↓ ↑"),
    ("keys.invert", "Ctrl+I"),
    ("keys.folder_sizes", "Alt+S"),
    ("keys.clipboard", "Ctrl+C X V"),
    ("keys.rename", "F2"),
    ("keys.trash", "Delete"),
    ("keys.undo", "Ctrl+Z / Ctrl+Shift+Z"),
    ("keys.new_file", "Shift+F10 menu"),
    ("keys.filter", "Ctrl+Shift+F"),
    ("keys.search", "Ctrl+F"),
    ("keys.path", "Ctrl+L"),
    ("keys.preview_pages", "Alt+PgDn · Alt+PgUp"),
    ("keys.tabs", "Ctrl+T / Ctrl+W"),
    ("keys.go_tab", "Alt+1–9 · Ctrl+Tab"),
    ("keys.move_tab", "Ctrl+Shift+PgUp / PgDn"),
    ("keys.menu", "Shift+F10 · right click"),
    ("keys.sheet", "F1"),
    ("keys.settings", "Ctrl+,"),
    ("keys.tag", "Ctrl+Alt+1–7 · Ctrl+Alt+0"),
    ("keys.type_ahead", "a–z"),
    ("keys.history", "Alt+← / Alt+→"),
    ("keys.reload", "F5"),
];

pub fn keymap_rows(preset: KeyPreset) -> Vec<(&'static str, &'static str)> {
    if preset == KeyPreset::Vim {
        return KEYMAP_ROWS.to_vec();
    }
    let classic = |label: &str| {
        CLASSIC_KEYS
            .iter()
            .find(|(key, _)| *key == label)
            .map(|(_, keys)| *keys)
    };
    let mut rows: Vec<(&str, &str)> = KEYMAP_ROWS
        .iter()
        .map(|(label, keys)| (*label, classic(label).unwrap_or(keys)))
        .collect();
    rows.extend(
        CLASSIC_KEYS
            .iter()
            .filter(|(label, _)| !KEYMAP_ROWS.iter().any(|(known, _)| known == label)),
    );
    rows
}
