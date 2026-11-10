use std::process::{Command, Stdio};

use crate::core::constants::network::{GIO_COMMAND, GIO_LOCALE, GIO_MOUNT, SCHEME_SEPARATOR};
use crate::core::constants::phones::{
    ACTIVATION_ROOT_PREFIX, GIO_LIST_FLAG, MTP_SCHEME, PHONE_SCHEMES, VOLUME_NAME_SEPARATOR,
    VOLUME_PREFIX,
};
use crate::features::phones::domain::entities::phone::Phone;
use crate::features::phones::domain::usecases::mtp_label::mtp_label;

pub fn gio_volumes() -> Vec<Phone> {
    let Ok(output) = Command::new(GIO_COMMAND)
        .args([GIO_MOUNT, GIO_LIST_FLAG])
        .env("LC_ALL", GIO_LOCALE)
        .env("LANGUAGE", GIO_LOCALE)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
    else {
        return Vec::new();
    };
    parse_volumes(&String::from_utf8_lossy(&output.stdout))
}

fn parse_volumes(listing: &str) -> Vec<Phone> {
    let mut phones: Vec<Phone> = Vec::new();
    let mut name: Option<String> = None;
    for line in listing.lines().map(str::trim) {
        if line.starts_with(VOLUME_PREFIX) {
            name = line
                .split_once(VOLUME_NAME_SEPARATOR)
                .map(|(_, found)| found.to_owned());
            continue;
        }
        let Some(uri) = line.strip_prefix(ACTIVATION_ROOT_PREFIX) else {
            continue;
        };
        let Some(found) = name.take().filter(|_| is_phone(uri)) else {
            continue;
        };
        if phones.iter().all(|phone| phone.uri != uri) {
            phones.push(Phone {
                name: phone_name(uri, found),
                uri: uri.to_owned(),
                mount: None,
            });
        }
    }
    phones
}

fn is_phone(uri: &str) -> bool {
    uri.split_once(SCHEME_SEPARATOR)
        .is_some_and(|(scheme, _)| PHONE_SCHEMES.contains(&scheme))
}

fn phone_name(uri: &str, volume: String) -> String {
    match uri.split_once(SCHEME_SEPARATOR) {
        Some((MTP_SCHEME, host)) => mtp_label(host),
        _ => volume,
    }
}
