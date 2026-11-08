use std::path::PathBuf;

use crate::core::enums::tags::tag_color::TagColor;
use crate::features::tags::domain::repositories::tag_store::TagStore;

fn has(names: &[String], tag: TagColor) -> bool {
    names
        .iter()
        .any(|name| TagColor::from_name(name) == Some(tag))
}

pub fn toggle_tag(store: &dyn TagStore, paths: &[PathBuf], tag: TagColor) -> Result<(), String> {
    let adding = !paths.iter().all(|path| has(&store.names(path), tag));
    for path in paths {
        let mut names: Vec<String> = store
            .names(path)
            .into_iter()
            .filter(|name| TagColor::from_name(name) != Some(tag))
            .collect();
        if adding {
            names.push(tag.name().to_string());
        }
        store.set_names(path, &names)?;
    }
    Ok(())
}

pub fn clear_tags(store: &dyn TagStore, paths: &[PathBuf]) -> Result<(), String> {
    for path in paths {
        let names: Vec<String> = store
            .names(path)
            .into_iter()
            .filter(|name| TagColor::from_name(name).is_none())
            .collect();
        store.set_names(path, &names)?;
    }
    Ok(())
}
