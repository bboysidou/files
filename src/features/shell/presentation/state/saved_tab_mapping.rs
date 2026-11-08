use crate::core::constants::settings::MAX_SAVED_HISTORY;
use crate::features::browser::domain::entities::navigation_history::{
    HistoryStop, NavigationHistory,
};
use crate::features::browser::presentation::state::tab_snapshot::TabSnapshot;
use crate::features::places::domain::entities::place::PlaceTarget;
use crate::features::settings::domain::entities::settings::{SavedStop, SavedTab, SavedTabState};

fn saved_target(target: &PlaceTarget) -> SavedTab {
    match target {
        PlaceTarget::Overview => SavedTab::Overview,
        PlaceTarget::Trash => SavedTab::Trash,
        PlaceTarget::Directory(path) => SavedTab::Directory { path: path.clone() },
    }
}

fn restored_target(tab: &SavedTab) -> Option<PlaceTarget> {
    match tab {
        SavedTab::Overview => Some(PlaceTarget::Overview),
        SavedTab::Trash => Some(PlaceTarget::Trash),
        SavedTab::Directory { path } => path.is_dir().then(|| PlaceTarget::Directory(path.clone())),
    }
}

fn saved_stops(stops: &[HistoryStop]) -> Vec<SavedStop> {
    let skip = stops.len().saturating_sub(MAX_SAVED_HISTORY);
    stops
        .iter()
        .skip(skip)
        .map(|stop| SavedStop {
            target: saved_target(&stop.target),
            focus: stop.focus.clone(),
        })
        .collect()
}

fn restored_stops(stops: &[SavedStop]) -> Vec<HistoryStop> {
    stops
        .iter()
        .filter_map(|stop| {
            Some(HistoryStop {
                target: restored_target(&stop.target)?,
                focus: stop.focus.clone(),
            })
        })
        .collect()
}

pub fn saved_tab(tab: &TabSnapshot) -> SavedTabState {
    SavedTabState {
        location: saved_target(&tab.location),
        back: saved_stops(tab.history.back_stops()),
        forward: saved_stops(tab.history.forward_stops()),
    }
}

pub fn restored_tab(tab: &SavedTabState) -> Option<TabSnapshot> {
    Some(TabSnapshot {
        location: restored_target(&tab.location)?,
        history: NavigationHistory::from_stops(
            restored_stops(&tab.back),
            restored_stops(&tab.forward),
        ),
    })
}
