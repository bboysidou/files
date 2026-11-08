use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(from = "SavedServerFormat")]
pub struct SavedServer {
    pub uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SavedServerFormat {
    Uri(String),
    Full {
        uri: String,
        #[serde(default)]
        name: Option<String>,
    },
}

impl From<SavedServerFormat> for SavedServer {
    fn from(format: SavedServerFormat) -> Self {
        match format {
            SavedServerFormat::Uri(uri) => Self { uri, name: None },
            SavedServerFormat::Full { uri, name } => Self { uri, name },
        }
    }
}
