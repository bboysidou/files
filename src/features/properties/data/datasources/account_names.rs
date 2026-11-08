use std::fs;

use crate::core::constants::files::{ACCOUNT_FIELD_SEPARATOR, GROUP_DATABASE, USER_DATABASE};

pub fn user_name(uid: u32) -> String {
    lookup(USER_DATABASE, uid)
}

pub fn group_name(gid: u32) -> String {
    lookup(GROUP_DATABASE, gid)
}

fn lookup(database: &str, id: u32) -> String {
    let wanted = id.to_string();
    fs::read_to_string(database)
        .ok()
        .and_then(|text| {
            text.lines().find_map(|line| {
                let mut fields = line.split(ACCOUNT_FIELD_SEPARATOR);
                let name = fields.next()?;
                (fields.nth(1)? == wanted).then(|| name.to_owned())
            })
        })
        .unwrap_or(wanted)
}
