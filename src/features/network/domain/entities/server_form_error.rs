#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ServerFormError {
    MissingHost,
    BadPort,
}

impl ServerFormError {
    pub fn label_key(self) -> &'static str {
        match self {
            ServerFormError::MissingHost => "server_edit.need_host",
            ServerFormError::BadPort => "server_edit.bad_port",
        }
    }
}
