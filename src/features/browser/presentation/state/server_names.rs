use gpui::Global;

use crate::features::network::domain::entities::saved_server::SavedServer;

#[derive(Default)]
pub struct ServerNames(pub Vec<SavedServer>);

impl Global for ServerNames {}
