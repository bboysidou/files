use std::any::Any;
use std::collections::HashMap;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

use futures::channel::mpsc::UnboundedSender;
use futures::channel::oneshot;
use zbus::blocking::connection;
use zbus::interface;
use zbus::zvariant::{ObjectPath, OwnedValue, Value};

use crate::core::constants::integration::{
    PORTAL_BUS_NAME, PORTAL_BUS_PATH, PORTAL_OPTION_ACCEPT, PORTAL_OPTION_DIRECTORY,
    PORTAL_OPTION_FOLDER, PORTAL_OPTION_MULTIPLE, PORTAL_OPTION_NAME, PORTAL_RESPONSE_CANCELLED,
    PORTAL_RESPONSE_OK, PORTAL_RESULT_URIS,
};
use crate::core::helpers::files::file_uri::file_uri;
use crate::features::picker::domain::entities::picker_request::{
    PickerMode, PickerRequest, PickerResult,
};
use crate::features::picker::domain::repositories::picker_portal::{PickerPortal, PickerReply};

type Options = HashMap<String, OwnedValue>;
type Answer = (u32, HashMap<String, OwnedValue>);

const NUL: u8 = 0;

struct PortalService {
    sender: UnboundedSender<(PickerRequest, PickerReply)>,
}

impl PortalService {
    fn flag(options: &Options, key: &str) -> bool {
        options
            .get(key)
            .and_then(|value| bool::try_from(value).ok())
            .unwrap_or(false)
    }

    fn text(options: &Options, key: &str) -> Option<String> {
        options
            .get(key)
            .and_then(|value| String::try_from(value.clone()).ok())
    }

    fn folder(options: &Options) -> Option<PathBuf> {
        let mut bytes = options
            .get(PORTAL_OPTION_FOLDER)
            .and_then(|value| Vec::<u8>::try_from(value.clone()).ok())?;
        while bytes.last() == Some(&NUL) {
            bytes.pop();
        }
        Some(PathBuf::from(OsString::from_vec(bytes)))
    }

    async fn ask(&self, mode: PickerMode, title: String, options: &Options) -> Answer {
        let request = PickerRequest {
            mode,
            title,
            multiple: Self::flag(options, PORTAL_OPTION_MULTIPLE),
            directory: Self::flag(options, PORTAL_OPTION_DIRECTORY),
            start_folder: Self::folder(options),
            suggested_name: Self::text(options, PORTAL_OPTION_NAME),
            accept_label: Self::text(options, PORTAL_OPTION_ACCEPT),
        };
        let (reply, answer) = oneshot::channel();
        if self.sender.unbounded_send((request, reply)).is_err() {
            return (PORTAL_RESPONSE_CANCELLED, HashMap::new());
        }
        match answer.await {
            Ok(PickerResult::Chosen(paths)) => {
                let uris: Vec<String> = paths.iter().map(|path| file_uri(path)).collect();
                let value = OwnedValue::try_from(Value::from(uris)).ok();
                (
                    PORTAL_RESPONSE_OK,
                    value
                        .map(|value| (PORTAL_RESULT_URIS.to_string(), value))
                        .into_iter()
                        .collect(),
                )
            }
            _ => (PORTAL_RESPONSE_CANCELLED, HashMap::new()),
        }
    }
}

#[interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl PortalService {
    #[allow(clippy::too_many_arguments)]
    async fn open_file(
        &self,
        _handle: ObjectPath<'_>,
        _app_id: String,
        _parent: String,
        title: String,
        options: Options,
    ) -> Answer {
        self.ask(PickerMode::Open, title, &options).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn save_file(
        &self,
        _handle: ObjectPath<'_>,
        _app_id: String,
        _parent: String,
        title: String,
        options: Options,
    ) -> Answer {
        self.ask(PickerMode::Save, title, &options).await
    }
}

pub struct DbusPickerPortal;

impl PickerPortal for DbusPickerPortal {
    fn serve(&self, sender: UnboundedSender<(PickerRequest, PickerReply)>) -> Option<Box<dyn Any>> {
        let connection = connection::Builder::session()
            .ok()?
            .name(PORTAL_BUS_NAME)
            .ok()?
            .serve_at(PORTAL_BUS_PATH, PortalService { sender })
            .ok()?
            .build()
            .ok()?;
        Some(Box::new(connection))
    }
}
