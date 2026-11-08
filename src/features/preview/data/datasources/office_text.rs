use std::path::Path;
use std::process::Stdio;

use crate::core::constants::preview::{
    ARCHIVE_COMMAND, OFFICE_EXTRACT_ARGUMENTS, OFFICE_MAX_BYTES, OFFICE_PARTS,
};
use crate::core::helpers::process::sandbox_access::SandboxAccess;
use crate::core::helpers::process::sandboxed_command::sandboxed_command;

const ENTITIES: [(&str, &str); 5] = [
    ("&lt;", "<"),
    ("&gt;", ">"),
    ("&quot;", "\""),
    ("&apos;", "'"),
    ("&amp;", "&"),
];

pub fn office_text(path: &Path, extension: &str) -> Option<String> {
    let (_, member, breaks) = OFFICE_PARTS
        .iter()
        .find(|(known, _, _)| known.eq_ignore_ascii_case(extension))?;
    let access = SandboxAccess {
        read: vec![path.to_path_buf()],
        write: Vec::new(),
    };
    let output = sandboxed_command(ARCHIVE_COMMAND, &access)
        .args(OFFICE_EXTRACT_ARGUMENTS)
        .arg(path)
        .arg(member)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()
        .filter(|output| output.status.success())?;
    let xml = String::from_utf8_lossy(&output.stdout[..output.stdout.len().min(OFFICE_MAX_BYTES)])
        .into_owned();
    let marked = breaks
        .iter()
        .fold(xml, |text, tag| text.replace(tag, &format!("{tag}\n")));
    Some(decode(&strip_tags(&marked)))
}

fn strip_tags(xml: &str) -> String {
    let mut plain = String::with_capacity(xml.len());
    let mut inside = false;
    for character in xml.chars() {
        match character {
            '<' => inside = true,
            '>' => inside = false,
            _ if !inside => plain.push(character),
            _ => {}
        }
    }
    plain
}

fn decode(text: &str) -> String {
    ENTITIES
        .iter()
        .fold(text.to_owned(), |text, (entity, plain)| {
            text.replace(entity, plain)
        })
}
