use crate::features::network::domain::entities::saved_server::SavedServer;
use crate::features::network::domain::usecases::server_key::server_key;
use crate::features::network::domain::usecases::server_label::server_label;

pub fn saved_server_name(label: &str, saved: &[SavedServer]) -> Option<String> {
    saved
        .iter()
        .find(|server| {
            server_label(&server.uri)
                .is_some_and(|address| server_key(&address) == server_key(label))
        })
        .and_then(|server| server.name.clone())
}
