use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyPreset {
    #[default]
    Vim,
    Classic,
}

impl KeyPreset {
    pub const ALL: [KeyPreset; 2] = [KeyPreset::Vim, KeyPreset::Classic];

    pub fn label_key(self) -> &'static str {
        match self {
            KeyPreset::Vim => "settings.keys.vim",
            KeyPreset::Classic => "settings.keys.classic",
        }
    }
}
