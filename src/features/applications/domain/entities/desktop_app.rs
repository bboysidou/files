#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesktopApp {
    pub id: String,
    pub name: String,
    pub exec: String,
    pub terminal: bool,
}
