#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ServerTab {
    Servers,
    New,
}

impl ServerTab {
    pub const ALL: [ServerTab; 2] = [ServerTab::Servers, ServerTab::New];

    pub fn label_key(self) -> &'static str {
        match self {
            ServerTab::Servers => "server_edit.tab_servers",
            ServerTab::New => "server_edit.tab_new",
        }
    }

    pub fn index(self) -> usize {
        match self {
            ServerTab::Servers => 0,
            ServerTab::New => 1,
        }
    }

    pub fn other(self) -> Self {
        match self {
            ServerTab::Servers => ServerTab::New,
            ServerTab::New => ServerTab::Servers,
        }
    }
}
