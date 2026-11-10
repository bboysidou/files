#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SshHost {
    pub alias: String,
    pub user: Option<String>,
}
