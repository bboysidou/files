use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::settings::domain::entities::settings::SavedTab;

pub fn saved_tab(target: &PlaceTarget) -> SavedTab {
    match target {
        PlaceTarget::Overview => SavedTab::Overview,
        PlaceTarget::Trash => SavedTab::Trash,
        PlaceTarget::Directory(path) => SavedTab::Directory { path: path.clone() },
    }
}

pub fn restored_target(tab: &SavedTab) -> Option<PlaceTarget> {
    match tab {
        SavedTab::Overview => Some(PlaceTarget::Overview),
        SavedTab::Trash => Some(PlaceTarget::Trash),
        SavedTab::Directory { path } => path.is_dir().then(|| PlaceTarget::Directory(path.clone())),
    }
}
