#[derive(Clone, PartialEq, Eq)]
pub struct NetworkCredentials {
    pub user: String,
    pub domain: String,
    pub password: String,
}
