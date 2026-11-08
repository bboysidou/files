#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BrowserEvent {
    ServerConnected {
        uri: String,
        replaces: Option<String>,
    },
}
