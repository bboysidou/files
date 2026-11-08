use std::path::{Path, PathBuf};

use crate::core::helpers::files::file_uri::file_uri;
use crate::features::applications::domain::usecases::tokenize_exec::tokenize_exec;

const FIELD_MARK: char = '%';
const ALL_FILES: &str = "%F";
const ALL_URIS: &str = "%U";

pub fn expand_exec(exec: &str, paths: &[PathBuf]) -> Option<Vec<String>> {
    let mut argv = Vec::new();
    for token in tokenize_exec(exec)? {
        match token.as_str() {
            ALL_FILES => argv.extend(paths.iter().map(|path| path.to_string_lossy().into_owned())),
            ALL_URIS => argv.extend(paths.iter().map(|path| file_uri(path))),
            _ => argv.extend(expand_token(&token, paths.first().map(PathBuf::as_path))),
        }
    }
    (!argv.is_empty()).then_some(argv)
}

fn expand_token(token: &str, first: Option<&Path>) -> Option<String> {
    let mut expanded = String::new();
    let mut chars = token.chars();
    while let Some(character) = chars.next() {
        if character != FIELD_MARK {
            expanded.push(character);
            continue;
        }
        match chars.next() {
            Some(FIELD_MARK) => expanded.push(FIELD_MARK),
            Some('f') => expanded.push_str(&first?.to_string_lossy()),
            Some('u') => expanded.push_str(&file_uri(first?)),
            _ => {}
        }
    }
    (!expanded.is_empty()).then_some(expanded)
}
