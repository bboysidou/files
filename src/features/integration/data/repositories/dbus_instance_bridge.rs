use std::any::Any;
use std::path::PathBuf;

use futures::channel::mpsc::UnboundedSender;
use zbus::blocking::{Connection, connection};
use zbus::interface;

use crate::core::constants::integration::{
    APP_BUS_INTERFACE, APP_BUS_NAME, APP_BUS_PATH, APP_OPEN_METHOD, FILE_MANAGER_BUS_NAME,
    FILE_MANAGER_BUS_PATH,
};
use crate::core::helpers::files::path_from_uri::path_from_uri;
use crate::features::integration::domain::entities::launch_request::LaunchRequest;
use crate::features::integration::domain::repositories::instance_bridge::InstanceBridge;

struct AppService {
    sender: UnboundedSender<LaunchRequest>,
}

#[interface(name = "io.github.bboysidou.Files")]
impl AppService {
    fn open(&self, path: String, reveal: bool) {
        let path = PathBuf::from(path);
        let request = if reveal {
            LaunchRequest::Reveal(path)
        } else {
            LaunchRequest::Open(path)
        };
        let _ = self.sender.unbounded_send(request);
    }
}

struct FileManagerService {
    sender: UnboundedSender<LaunchRequest>,
}

impl FileManagerService {
    fn send_all(&self, uris: Vec<String>, wrap: fn(PathBuf) -> LaunchRequest) {
        for path in uris.iter().filter_map(|uri| path_from_uri(uri)) {
            let _ = self.sender.unbounded_send(wrap(path));
        }
    }
}

#[interface(name = "org.freedesktop.FileManager1")]
impl FileManagerService {
    fn show_folders(&self, uris: Vec<String>, _startup_id: String) {
        self.send_all(uris, LaunchRequest::Open);
    }

    fn show_items(&self, uris: Vec<String>, _startup_id: String) {
        self.send_all(uris, LaunchRequest::Reveal);
    }

    fn show_item_properties(&self, uris: Vec<String>, _startup_id: String) {
        self.send_all(uris, LaunchRequest::Reveal);
    }
}

pub struct DbusInstanceBridge;

impl InstanceBridge for DbusInstanceBridge {
    fn forward(&self, request: &LaunchRequest) -> bool {
        let (path, reveal) = match request {
            LaunchRequest::Open(path) => (path.clone(), false),
            LaunchRequest::Reveal(path) => (path.clone(), true),
            LaunchRequest::Window => (PathBuf::new(), false),
            LaunchRequest::SetDefault(_) | LaunchRequest::Portal | LaunchRequest::PortalSetup => {
                return false;
            }
        };
        let Ok(connection) = Connection::session() else {
            return false;
        };
        connection
            .call_method(
                Some(APP_BUS_NAME),
                APP_BUS_PATH,
                Some(APP_BUS_INTERFACE),
                APP_OPEN_METHOD,
                &(path.to_string_lossy().into_owned(), reveal),
            )
            .is_ok()
    }

    fn serve(
        &self,
        sender: UnboundedSender<LaunchRequest>,
        claim_file_manager: bool,
    ) -> Option<Box<dyn Any>> {
        let app = connection::Builder::session()
            .ok()?
            .name(APP_BUS_NAME)
            .ok()?
            .serve_at(
                APP_BUS_PATH,
                AppService {
                    sender: sender.clone(),
                },
            )
            .ok()?
            .build()
            .ok()?;
        let file_manager = claim_file_manager
            .then(|| {
                connection::Builder::session()
                    .ok()?
                    .name(FILE_MANAGER_BUS_NAME)
                    .ok()?
                    .serve_at(FILE_MANAGER_BUS_PATH, FileManagerService { sender })
                    .ok()?
                    .build()
                    .ok()
            })
            .flatten();
        Some(Box::new((app, file_manager)))
    }
}
