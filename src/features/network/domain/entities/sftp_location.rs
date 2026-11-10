#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SftpLocation {
    pub host: String,
    pub user: Option<String>,
    pub port: Option<String>,
    pub remote_path: String,
}
